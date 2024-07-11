[dependencies]
reqwest = { version = "0.11", features = ["json"] }
ndarray = "0.15"
ndarray-rand = "0.15"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Optional: For TF-IDF vectorization
rust-tokenizers = "0.10"
tfidf = "0.2"
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Hit {
    content: String,
    // Add more fields if needed
}

fn get_documents(index_name: &str, client: &Client) -> Vec<String> {
    let url = format!("http://localhost:9200/{}/_search", index_name);
    let response = client.get(&url).send().unwrap();

    let hits: HitsResponse = response.json().unwrap();
    hits.hits.into_iter().map(|hit| hit.content).collect()
}

#[derive(Deserialize)]
struct HitsResponse {
    hits: Vec<Hit>,
}

use tfidf::TfIdfVectorizer;

fn tfidf_vectorization(documents: &[String]) -> Vec<Vec<f64>> {
    let vectorizer = TfIdfVectorizer::fit(documents);
    let tfidf_matrix = vectorizer.transform(documents);
    tfidf_matrix.to_vec()
}

use linfa::clustering::{KMeans, KMeansHyperParams};

fn kmeans_clustering(tfidf_matrix: &[Vec<f64>], num_clusters: usize) -> Vec<usize> {
    let params = KMeansHyperParams::new(num_clusters).max_n_iterations(100);
    let model = KMeans::params(params).fit(&tfidf_matrix).unwrap();
    model.predict(tfidf_matrix)
}

use serde_json::json;

fn store_clusters(index_name: &str, cluster_labels: &[usize], client: &Client) {
    let actions: Vec<serde_json::Value> = cluster_labels.iter().enumerate().map(|(doc_idx, label)| {
        json!({
            "index": {
                "_index": index_name,
                "_id": doc_idx
            }
        })
    }).collect();

    // Prepare bulk update request
    let mut bulk_request = String::new();
    for action in actions {
        bulk_request.push_str(&serde_json::to_string(&action).unwrap());
        bulk_request.push_str("\n");
        bulk_request.push_str(&serde_json::to_string(&json!({ "cluster_label": label })).unwrap());
        bulk_request.push_str("\n");
    }

    // Send bulk update request
    let url = format!("http://localhost:9200/_bulk");
    let _response = client.post(&url).body(bulk_request).send().unwrap();
}


fn main() {
    let client = reqwest::blocking::Client::new();
    let index_name = "your_index_name";

    // Step 1: Retrieve documents from Elasticsearch
    let documents = get_documents(index_name, &client);

    // Step 2: Perform TF-IDF vectorization
    let tfidf_matrix = tfidf_vectorization(&documents);

    // Step 3: Perform K-means clustering
    let num_clusters = 5;
    let cluster_labels = kmeans_clustering(&tfidf_matrix, num_clusters);

    // Step 4: Store cluster information back into Elasticsearch
    store_clusters(index_name, &cluster_labels, &client);
}



use reqwest::blocking::Client;
use serde::Deserialize;
use tfidf::TfIdfVectorizer;
use linfa::clustering::{KMeans, KMeansParams};
use ndarray::Array2;
use serde_json::json;
use std::error::Error;

#[derive(Deserialize)]
struct Hit {
    _source: Source,
}

#[derive(Deserialize)]
struct Source {
    content: String,
}

#[derive(Deserialize)]
struct HitsResponse {
    hits: InnerHits,
}

#[derive(Deserialize)]
struct InnerHits {
    hits: Vec<Hit>,
}

fn get_documents(index_name: &str, client: &Client) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("http://localhost:9200/{}/_search", index_name);
    let response = client.get(&url).send()?;
    let hits: HitsResponse = response.json()?;
    let documents: Vec<String> = hits.hits.hits.into_iter().map(|hit| hit._source.content).collect();
    Ok(documents)
}

fn tfidf_vectorization(documents: &[String]) -> Array2<f64> {
    let vectorizer = TfIdfVectorizer::fit(documents);
    let tfidf_matrix = vectorizer.transform(documents);
    let nrows = tfidf_matrix.len();
    let ncols = tfidf_matrix[0].len();
    let mut flat_matrix: Vec<f64> = Vec::with_capacity(nrows * ncols);
    for row in tfidf_matrix {
        flat_matrix.extend(row);
    }
    Array2::from_shape_vec((nrows, ncols), flat_matrix).unwrap()
}

fn kmeans_clustering(tfidf_matrix: &Array2<f64>, num_clusters: usize) -> Vec<usize> {
    let params = KMeansParams::new(num_clusters).max_n_iterations(100);
    let model = KMeans::fit(tfidf_matrix, params).unwrap();
    model.predict(tfidf_matrix).into_raw_vec()
}

fn store_clusters(index_name: &str, cluster_labels: &[usize], client: &Client) -> Result<(), Box<dyn Error>> {
    let actions: Vec<serde_json::Value> = cluster_labels.iter().enumerate().map(|(doc_idx, label)| {
        json!({
            "update": {
                "_index": index_name,
                "_id": doc_idx,
            }
        })
    }).collect();

    let mut bulk_request = String::new();
    for action in actions {
        bulk_request.push_str(&serde_json::to_string(&action)?);
        bulk_request.push_str("\n");
        bulk_request.push_str(&serde_json::to_string(&json!({ "doc": { "cluster_label": label } }))?);
        bulk_request.push_str("\n");
    }

    let url = format!("http://localhost:9200/_bulk");
    client.post(&url).body(bulk_request).send()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let index_name = "your_index_name";

    // Step 1: Retrieve documents from Elasticsearch
    let documents = get_documents(index_name, &client)?;

    // Step 2: Perform TF-IDF vectorization
    let tfidf_matrix = tfidf_vectorization(&documents);

    // Step 3: Perform K-means clustering
    let num_clusters = 5;
    let cluster_labels = kmeans_clustering(&tfidf_matrix, num_clusters);

    // Step 4: Store cluster information back into Elasticsearch
    store_clusters(index_name, &cluster_labels, &client)?;

    Ok(())
}



[dependencies]
reqwest = { version = "0.11", features = ["json"] }
ndarray = "0.15"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rust-tokenizers = "0.10"
tfidf = "0.2"
linfa = "0.6"
linfa-clustering = "0.6"


use reqwest::blocking::Client;
use serde::Deserialize;
use tfidf::TfIdfVectorizer;
use ndarray::Array2;
use ndarray::prelude::*;
use linfa_clustering::HierarchicalClustering;
use linfa::traits::Transformer;
use serde_json::json;
use std::error::Error;

#[derive(Deserialize)]
struct Hit {
    _source: Source,
}

#[derive(Deserialize)]
struct Source {
    content: String,
}

#[derive(Deserialize)]
struct HitsResponse {
    hits: InnerHits,
}

#[derive(Deserialize)]
struct InnerHits {
    hits: Vec<Hit>,
}

fn get_documents(index_name: &str, client: &Client) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("http://localhost:9200/{}/_search", index_name);
    let response = client.get(&url).send()?;
    let hits: HitsResponse = response.json()?;
    let documents: Vec<String> = hits.hits.hits.into_iter().map(|hit| hit._source.content).collect();
    Ok(documents)
}

fn tfidf_vectorization(documents: &[String]) -> Array2<f64> {
    let vectorizer = TfIdfVectorizer::fit(documents);
    let tfidf_matrix = vectorizer.transform(documents);
    let nrows = tfidf_matrix.len();
    let ncols = tfidf_matrix[0].len();
    let mut flat_matrix: Vec<f64> = Vec::with_capacity(nrows * ncols);
    for row in tfidf_matrix {
        flat_matrix.extend(row);
    }
    Array2::from_shape_vec((nrows, ncols), flat_matrix).unwrap()
}

fn hierarchical_clustering(tfidf_matrix: &Array2<f64>, num_clusters: usize) -> Vec<usize> {
    let model = HierarchicalClustering::params(num_clusters)
        .euclidean()
        .fit(tfidf_matrix)
        .unwrap();
    model.predict(tfidf_matrix).to_vec()
}

fn store_clusters(index_name: &str, cluster_labels: &[usize], client: &Client) -> Result<(), Box<dyn Error>> {
    let actions: Vec<serde_json::Value> = cluster_labels.iter().enumerate().map(|(doc_idx, label)| {
        json!({
            "update": {
                "_index": index_name,
                "_id": doc_idx,
            }
        })
    }).collect();

    let mut bulk_request = String::new();
    for action in actions {
        bulk_request.push_str(&serde_json::to_string(&action)?);
        bulk_request.push_str("\n");
        bulk_request.push_str(&serde_json::to_string(&json!({ "doc": { "cluster_label": label } }))?);
        bulk_request.push_str("\n");
    }

    let url = format!("http://localhost:9200/_bulk");
    client.post(&url).body(bulk_request).send()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let index_name = "your_index_name";

    // Step 1: Retrieve documents from Elasticsearch
    let documents = get_documents(index_name, &client)?;

    // Step 2: Perform TF-IDF vectorization
    let tfidf_matrix = tfidf_vectorization(&documents);

    // Step 3: Perform hierarchical clustering
    let num_clusters = 5;
    let cluster_labels = hierarchical_clustering(&tfidf_matrix, num_clusters);

    // Step 4: Store cluster information back into Elasticsearch
    store_clusters(index_name, &cluster_labels, &client)?;

    Ok(())
}
