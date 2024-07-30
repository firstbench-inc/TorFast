mod crawler;
mod fetcher;
mod parser;
mod poster;
mod test;
// mod tests;

extern crate markup5ever_rcdom as rcdom;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use std::{env, thread};
use tokio::sync::Notify;

#[tokio::main()]
async fn main() -> Result<(), reqwest::Error> {
    let seedlist: [&str; 252] = [
        "http://torlinkv7cft5zhegrokjrxj2st4hcimgidaxdmcmdpcrnwfxrr2zxqd.onion/",
        "http://fvrifdnu75abxcoegldwea6ke7tnb3fxwupedavf5m3yg3y2xqyvi5qd.onion/",
        "http://zqktlwiuavvvqqt4ybvgvi7tyo4hjl5xgfuvpdf6otjiycgwqbym2qad.onion/wiki/index.php/Main_Page",
        "http://3bbad7fauom4d6sgppalyqddsqbf5u5p56b5k5uk2zxsy3d6ey2jobad.onion/discover",
        "http://tt3j2x4k5ycaa5zt.onion/",
        "http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion/address/",
        "http://juhanurmihxlp77nkq276byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion/add/onionsadded/",
        "http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=19&pg=1",
        "http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=20&pg=1&lang=en",
        "http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=7&pg=1&lang=en",
        "https://github.com/alecmuffett/real-world-onion-sites",
        "http://o54hon2e2vj6c7m3aqqu6uyece65by3vgoxxhlqlsvkmacw6a7m7kiad.onion",
        "http://incognitox3vs5grdnmh52k35m64vib5fsbdrxzilujjptiqzeyrxhid.onion",
        "https://duckduckgogg42xjoc72x3sjasowoarfbgcmvfimaftt6twagswzczad.onion",
        "https://protonmailrmez3lotccipshtkleegetolb73fuirgj7r4o4vfu7ozyd.onion",
        "http://p53lf57qovyuvwsc6xnrppyply3vtqm7l6pcobkmyqsiofyeznfu5uqd.onion",
        "http://nexusb2l7hog66bnzz5msrz4m5qxj7jbi7aah3r65uzydy5mew2fu3id.onion/",
        "http://4pt4axjgzmm4ibmxplfiuvopxzf775e5bqseyllafcecryfthdupjwyd.onion",
        "http://lpiyu33yusoalp5kh3f4hak2so2sjjvjw5ykyvu2dulzosgvuffq6sad.onion",
        "http://torbox36ijlcevujx7mjb4oiusvwgvmue7jfn2cvutwa6kl6to3uyqad.onion",
        "http://rrlm2f22lpqgfhyydqkxxzv6snwo5qvc2krjt2q557l7z4te7fsvhbid.onion",
        "http://coinlnkn5qg5or6ixlgv5lxjq5ugvktpvikdgalop2u53cocw65q6oid.onion/",
        "http://enxx3byspwsdo446jujc52ucy2pf5urdbhqw3kbsfhlfjwmbpj5smdad.onion",
        "http://wmj5kiic7b6kjplpbvwadnht2nh2qnkbnqtcv3dyvpqtz7ssbssftxid.onion",
        "http://ddosxlvzzow7scc7egy75gpke54hgbg2frahxzaw6qq5osnzm7wistid.onion",
        "http://lgh3eosuqrrtvwx3s4nurujcqrm53ba5vqsbim5k5ntdpo33qkl7buyd.onion",
        "http://torzon4kv5swfazrziqvel2imhxcckc4otcvopiv5lnxzpqu4v4m5iyd.onion",
        "http://alphabay522szl32u4ci5e3iokdsyth56ei7rwngr2wm7i5jo54j2eid.onion",
        "http://2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion",
        "https://www.bbcnewsd73hkzno2ini43t4gblxvycyac5aw4gnv7t2rccijh7745uqd.onion",
        "http://abacuseeettcn3n2zxo7tqy5vsxhqpha2jtjqs7cgdjzl2jascr4liad.onion/",
        "http://vww6ybal4bd7szmgncyruucpgfkqahzddi37ktceo3ah7ngmcopnpyyd.onion/",
        "http://ncidetfs7banpz2d7vpndev5somwoki5vwdpfty2k7javniujekit6ad.onion",
        "http://blkchairbknpn73cfjhevhla7rkp4ed5gg2knctvv7it4lioy22defid.onion",
        "http://superxxx2daymhfxbxfzlg2zevkwqyvisngvphzjlwavgwl4bzn5rvqd.onion/",
        "https://www.nytimesn7cgmftshazwhfgzm37qxb44r64ytbb2dj3x62d2lljsciiyd.onion",
        "http://python7xnsayxuxvoheh5372vwrufvxgddydx33gnfqzmpz5knuj7cid.onion",
        "https://njallalafimoej5i4eg7vlnqjvmb6zhdh27qxcatdn647jtwwwui3nad.onion/",
        "http://zkaan2xfbuxia2wpf7ofnkbz6r5zdbbvxbunvp5g2iebopbfc4iqmbad.onion",
        "http://nehdddktmhvqklsnkjqcbpmb63htee2iznpcbs5tgzctipxykpj6yrid.onion",
        "http://bohemiaobko4cecexkj5xmlaove6yn726dstp5wfw4pojjwp6762paqd.onion",
        "http://stormwayszuh4juycoy4kwoww5gvcu2c4tdtpkup667pdwe4qenzwayd.onion",
        "http://rhc62vwjq25n52umfmfkm4yq7yuxwuk6bjduyvzjzhb3cyhp2q65m2qd.onion/",
        "http://3w63t4tdhijukufciaefkzjpgt7tjejbnubwckxvtdeuhesbgarc5vqd.onion",
        "http://o455kwz35ukwqp5zpqa3fs4vi44mhyeliiadpgjb4ele2qlyucjxtrid.onion", 
        "http://plnemlsyla6h5t3nuoz2algzmy635ceuendnjwsmhwn2os5fxahshiad.onion/",
        "http://galaxy3yrfbwlwo72q3v2wlyjinqr2vejgpkxb22ll5pcpuaxlnqjiid.onion/",
        "https://codexygpw5vdngnjflqlznr65ltfl7r3pxswphbzl6yywn3l7ew5nkqd.onion", 
        "http://7ukmkdtyxdkdivtjad57klqnd3kdsmq6tp45rrsxqnu76zzv3jvitlqd.onion/", 
        "http://protonmailrmez3lotccipshtkleegetolb73fuirgj7r4o4vfu7ozyd.onion", 
        "http://facebookwkhpilnemxj7asaniu7vnjjbiltxjqhye3mhbshg7kx5tfyd.onion", 
        "https://majestictfvnfjgo5hqvmuzynak4kjl5tjs3j5zdabawe6n2aaebldad.onion/", 
        "http://pqtymsmtws5ms6pqqput7pe2ubv5ijyzdi7gwnjjuwibogwglelauxqd.onion/", 
        "http://lib.anarcopym4ckplfg3uljfj37y27ko3vrnrp43msyu5k5rp3h46wtf7yd.onion", 
        "http://i3xi5qxvbrngh3g6o7czwjfxwjzigook7zxzjmgwg5b7xnjcn5hzciad.onion/", 
        "http://conet72gphb3524hwjshszuogongo5yzmgcdw7342rulwpuezzc3gzyd.onion/", 
        "http://5ety7tpkim5me6eszuwcje7bmy25pbtrjtue7zkqqgziljwqy3rrikqd.onion/", 
        "http://w27irt6ldaydjoacyovepuzlethuoypazhhbot6tljuywy52emetn7qd.onion/", 
        "http://photodakj4vrljvu55bf6s7acpdbzvtvxpkd65gp6jbhquhbsvebodqd.onion/", 
        "http://w4ljqtyjnxinknz4hszn4bsof7zhfy5z2h4srfss4vvkoikiwz36o3id.onion/", 
        "http://tor66sewebgixwhcqfnp5inzp5x5uohhdy3kvtnyfxc2e5mxiuh34iid.onion/", 
        "http://fbbtyosausdfn3ogl66spqj5prnu3tscokaijcimd3hwszoe5hc4r4yd.onion/", 
        "http://pflujznptk5lmuf6xwadfqy6nffykdvahfbljh7liljailjbxrgvhfid.onion/", 
        "http://empirebhczt2s4yprurhtqvkvnt6rvxqlaxfyniqec643vird7gi7cid.onion", 
        "http://rambleeeqrhty6s5jgefdfdtc6tfgg4jj6svr4jpgk4wjtg3qshwbaad.onion/", 
        "http://xjfbpuj56rdazx4iolylxplbvyft2onuerjeimlcqwaihp3s6r4xebqd.onion", 
        "http://ds666p4hsmq67dcroyyifftdadprzkxl3pcqe5qlkkcticbeahfrzzad.onion/", 
        "http://k6m3fagp4w4wspmdt23fldnwrmknse74gmxosswvaxf3ciasficpenad.onion", 
        "http://anopicuscaszyis5xxyz623snrh4ikvw4mai3lf2qk37vfm22b52chyd.onion/", 
        "http://diasporg5tj4xz5mxkd5qnrppo7tbb6ynk2gtmjw5lmz6mtbesj3k6id.onion/", 
        "http://g66ol3eb5ujdckzqqfmjsbpdjufmjd5nsgdipvxmsh7rckzlhywlzlqd.onion/", 
        "http://vfdvqflzfgwnejh6rrzjnuxvbnpgjr4ursv4moombwyauot5c2z6ebid.onion/", 
        "http://wikiv2z7bogl633j4ok2fs3a3ht5f45gpjtiasmqwuxacclxek4u47qd.onion", 
        "http://2z7le7dgf6jffucj5gc6xv6sbtg7ajvsyoflxthekc3ereoensl4w3yd.onion/", 
        "http://owlzyj4to3l5daq6edgsgp5z4lh4tzlnms4z6jv6xdtkily77j4b3byd.onion/beginner-guides/", 
        "http://srmdho5coxk4c5ny3mtmnuqrjp34fdotq26gtiuwyxf6efoi6km4t6ad.onion/", 
        "http://qwikxx2erhx6qrymued6ox2qkf2yeogjwypqvzoif4fqkljixasr6oid.onion/", 
        "http://pfvhvoy2rdecvuruco37mcocipoq77bnp5hkro4evvkrimq4z3t2tiid.onion/", 
        "http://i43v6kjgdvh6bg6mvmfx6aonspvqngz2mexvclsueopy3b7szc47zyqd.onion/", 
        "http://nrud2lfujxlaw4wk2w5bllngqalmnui5rzktqklvvjd6rwek5yhhydqd.onion/", 
        "http://roleplay3zofkrw3m6z2wyvuxwdvccj5besxqxi2rzjn7xd2u46qz5qd.onion", 
        "http://devilqyh54mkpin4s2hpqsxowj3ogorolaoigh4kepawyi5njsuvc6qd.onion/", 
        "http://rnsm777cdsjrsdlbs4v5qoeppu3px6sb2igmh53jzrx7ipcrbjz5b2ad.onion/", 
        "http://reycdxyc24gf7jrnwutzdn3smmweizedy7uojsa7ols6sflwu25ijoyd.onion/", 
        "http://lu4aakwcq2b5dgqufc464fireyxvjb7o2rcwmojtjwuxlmwcyi345gyd.onion/", 
        "http://4wqk5mz57fgtwfgpevj3e6ri42o4ln3qoyzcqtipvrdah7qalnhxwkqd.onion/", 
        "http://tormailpout6wplxlrkkhjj2ra7bmqaij5iptdmhhnnep3r6f27m2yid.onion", 
        "http://t43fsf65omvf7grt46wlt2eo5jbj3hafyvbdb7jtr2biyre5v24pebad.onion/", 
        "http://666sspeavdj3p7buxuyc6lhs26qncfqybihkwsds3pepimgch3z2ucyd.onion/", 
        "http://chemi5wtn2hs27wlwgaosi663wswrutofqzhvrjb2ogtxpb42gezebqd.onion", 
        "http://mail2torjgmxgexntbrmhvgluavhj7ouul5yar6ylbvjkxwqf6ixkwyd.onion/", 
        "http://wasabiukrxmkdgve5kynjztuovbg43uxcbcxn6y2okcrsg7gb6jdmbad.onion", 
        "http://envoyyvazgz2wbkq65md7dcqsgmujmgksowhx2446yep7tgnpfvlxbqd.onion/", 
        "http://uoxqi4lrfqztugili7zzgygibs4xstehf5hohtkpyqcoyryweypzkwid.onion/", 
        "http://hxuzjtocnzvv5g2rtg2bhwkcbupmk7rclb6lly3fo4tvqkk5oyrv3nid.onion/", 
        "http://educate6mw6luxyre24uq3ebyfmwguhpurx7ann635llidinfvzmi3yd.onion/", 
        "http://monerotoruzizulg5ttgat2emf4d6fbmiea25detrmmy7erypseyteyd.onion", 
        "http://killerord6ztyuaav3s4xp6i4halek3dxxvboscgej57my5nl2fp2kqd.onion/", 
        "https://er3n3jnvoyj2t37yngvzr35b6f4ch5mgzl3i6qlkvyhzmaxo62nlqmqd.onion/", 
        "https://dydaofm5uefuulnzb63uh6coodgbxlgndk4eosoopbekebttkdxshlyd.onion", 
        "https://xw226dvxac7jzcpsf4xb64r4epr6o5hgn46dxlqk7gnjptakik6xnzqd.onion/", 
        "https://edupuyl7thgti3a33hzy3lh5shnkdar7rxmi5ronjlodnyoadkngk6qd.onion/", 
        "http://suprbaydvdcaynfo4dgdzgxb4zuso7rftlil5yg5kqjefnw4wq4ulcad.onion/", 
        "http://ygx5ek4op42ljsxcbg6jsx4k255eggn3cj65z5xkf4iei5frwvvkr6yd.onion/", 
        "http://6oyq2t37y76qjfggrcpbjb3kmyvjtvd2djxa7lh3dvyqnyz2ujd6otqd.onion/", 
        "http://santat7kpllt6iyvqbr7q4amdv6dzrh6paatvyrzl7ry3zm72zigf4ad.onion/", 
        "http://n7arfmj2ycewhreuctme5yrm2qjxrcguwhtvbgyv3qsgsjckafl6wjyd.onion", 
        "http://blkhatjxlrvc5aevqzz5t6kxldayog6jlx5h7glnu44euzongl4fh5ad.onion", 
        "http://anarcopym4ckplfg3uljfj37y27ko3vrnrp43msyu5k5rp3h46wtf7yd.onion/", 
        "http://onnin36kbcwchiacwjhl57m7wylygmulmlazpfg3qupucgjgq7blldid.onion/", 
        "http://satoshinrwezrmas2f2epg2mhlm4nefnl5fbbkatml3yhffyzq2ayyad.onion/", 
        "http://torbox36ijlcevujx7mjb4oiusvwgvmue7jfn2cvutwa6kl6to3uyqad.onion", 
        "http://breachedu76kdyavc6szj6ppbplfqoz3pgrk3zw57my4vybgblpfeayd.onion/", 
        "http://theendgtso35ir6ngdtyhgtjhhbbprmkzl74gt5nyeu3ocr34sfa67yd.onion/", 
        "http://giftto33ep564ztvpc6652xt4vkupphghcvtxqwpxi6gq5k2fmjd4zid.onion", 
        "http://p53lf57qovyuvwsc6xnrppyply3vtqm7l6pcobkmyqsiofyeznfu5uqd.onion", 
        "http://anonblogd4pcarck2ff6qlseyawjljaatp6wjq6rqpet2wfuoom42kyd.onion/", 
        "http://onnii6niq53gv3rvjpi7z5axkasurk2x5w5lwliep4qyeb2azagxn4qd.onion/", 
        "http://hacks4ucdmmx733hkrjtzb3qcxidhwbqe2mb3beb3yybbwfkg2zzxdad.onion/", 
        "http://indexzz7n3cq4slh5bh2lcctmiwk2y7epxjvkpyaemtuat2alprveyid.onion/", 
        "http://cards7ndxk4fuctkgwmeq46gx6bhzt57sg4l2nbwa2p3vjnvq4trhkad.onion", 
        "http://gsyorszxsxt57kwa5hnmqvgqu22en6rgif37uormbu6nafdxja62p5qd.onion/", 
        "http://fyonionq6mktpre6ustimnlqxbl5g757khnbgpyxt46lxx7umcy6ptid.onion/", 
        "https://rfaorg4ob4vj6n45djaaxkkxye4wiwimucbkvzvdsvwf3ebw2ale77yd.onion", 
        "http://trutube54vqa3baer2kc7aggfc5qstxkgcmzm4qxxzfaxdsihu3znzid.onion/", 
        "http://5n4qdkw2wavc55peppyrelmb2rgsx7ohcb2tkxhub2gyfurxulfyd3id.onion", 
        "http://spygame5awoookfmfhda7jfqimwyfxicjkn3wc4v3oozyno7sqv2axid.onion/", 
        "http://deepa2kol4ur4wkzpmjf5rf7lvsflzisslnrnr2n7goaebav4j6w7zyd.onion/", 
        "http://lqxqynuoppwk5w5qoybyetwsrtru6dhgn2i77s2vfra6c5cn2u3scgyd.onion/", 
        "http://u2ljln34n55et3xp2wiewybvwr74p4rmcsf2zrravtqj3turtbyjd4id.onion/", 
        "http://n4celbknwkn4twryohqzdko3txv7p3s7kgalvdglapd74vg3yrzin6id.onion/", 
        "https://www.voaturkqgmfs6ufdxthgkboec6fsadctaacqceksiji25r4id2xdl2ad.onion/", 
        "http://refuge3noitqegmmjericvur54ihyj7tsfyfwdblitaeaqu2koi7iuqd.onion", 
        "http://vpocsdxjwaodp73xm65pscydeidt3uap2lftcuuhxpznjigtuzx5pqad.onion/", 
        "http://bbng47x4sdy3fralpblqcvgmzmlljah72vys6ip2wtzu6wnorggartyd.onion/", 
        "http://wbi67emmdx6i6rcr6nnk3hco3nrvdc2juxrbvomvt6nze5afjz6pgtad.onion/", 
        "http://venus7rnkrbtdctbdtmvydn5kvjiaqmf2kgtzfup4muavc6wx4clwyqd.onion/", 
        "http://commudazrdyhbullltfdy222krfjhoqzizks5ejmocpft3ijtxq5khqd.onion/", 
        "http://azrailgggtfmgboqumc3li3bltcj36xoxrzum7ofi7lqhcau2lsp2gid.onion/", 
        "http://darkfailenbsdla5mal2mxn2uz66od5vtzd5qozslagrfzachha3f3id.onion/", 
        "http://marxists3va6eopxoeiegih3iyex2zg3tmace7afbxjqlabmranzjjad.onion/", 
        "http://dkforestseeaaq2dqz2uflmlsybvnq2irzn4ygyvu53oazyorednviid.onion/", 
        "http://bcloudwenjxgcxjh6uheyt72a5isimzgg4kv5u74jb2s22y3hzpwh6id.onion/", 
        "http://g7ejphhubv5idbbu3hb3wawrs5adw7tkx7yjabnf65xtzztgg4hcsqqd.onion/", 
        "http://wpzzhvw6q32pneau3rzsa5h7tzl7fgswya3cgtmu5rpesd725bii3cad.onion/", 
        "http://kf5waqrpn3vlk3fw7hs7fvy2uaa5snvtrnpss7lfbypb3ag2u7fuqoyd.onion/", 
        "http://vww6ybal4bd7szmgncyruucpgfkqahzddi37ktceo3ah7ngmcopnpyyd.onion/", 
        "http://anonimjjzzccjgmxgowzgbp3eocjljdxz2lban7ozwmdrudopezvugqd.onion", 
        "http://germania7zs27fu3gi76wlr5rd64cc2yjexyzvrbm4jufk7pibrpizad.onion/", 
        "http://leaklook7mhf6yfp6oyoyoe6rk7gmpuv2wdk5hgwhtu2ym5f4zvit7yd.onion", 
        "http://4eqjmxcalg5wel3p2wko4mewy3dxhkp2epsgtr4ds5ufxvu4rzv6hbqd.onion/", 
        "http://liqr2cbsjzxmpw6savgh274tuzl34x6cd56h7m7ceatnrokveffm66ad.onion/", 
        "http://dizqjnlw2wo7j3fu7j745izzkpeaalxulypuwobpuiev3ut24hga6qad.onion/", 
        "http://ncc3uu62rc4oacmlfkkxetwn3lgkgq7f65lkg6uedsl7xvbyaljiiead.onion/", 
        "http://zwf5i7hiwmffq2bl7euedg6y5ydzze3ljiyrjmm7o42vhe7ni56fm7qd.onion/", 
        "http://ovgl57qc3a5abwqgdhdtssvmydr6f6mjz6ey23thwy63pmbxqmi45iid.onion/", 
        "http://tenebrsi5ougn2slw7i47emfuzuebulhllycuxpihozb7wrpqslfwvqd.onion/", 
        "http://vap3miljiaw72s6k4jto2xrhecodjv2hhar5cd45gjbgmutkx7q6s2yd.onion/", 
        "http://blkchairbknpn73cfjhevhla7rkp4ed5gg2knctvv7it4lioy22defid.onion", 
        "http://rcuhe6pk7mbmsjk7bwyja5etvjhjvzmc724rnf3piamemvawoi44z7qd.onion/", 
        "http://pwjbiw5aub24nhztiw24mq26gwubjzfpjls5jerrachczfffvd25boyd.onion/", 
        "https://duckduckgogg42xjoc72x3sjasowoarfbgcmvfimaftt6twagswzczad.onion", 
        "http://iqqioovatewqdieuvfamy6vtihmzqzyn7jvqlkvvuyeiakc5ls272bid.onion/", 
        "http://enxx3byspwsdo446jujc52ucy2pf5urdbhqw3kbsfhlfjwmbpj5smdad.onion/", 
        "http://jt3it42syepkpcplixdxcnv2vz53cpk27lek2pptqlgrnqyb7cuqojqd.onion", 
        "http://hqvv7qwjs72kh65qsmwj3ifwtdgewiiiip62rkvmaq6q7zhhqrta2wid.onion/", 
        "http://nq4zyac4ukl4tykmidbzgdlvaboqeqsemkp4t35bzvjeve6zm2lqcjid.onion/", 
        "http://bru56n3gn4gqbkgs4gf2b6i2lgz4emw67kzneamfvinbm3qf4fza7vad.onion/", 
        "http://tssa3yo5xfkcn4razcnmdhw5uxshx6zwzngwizpyf7phvea3gccrqbad.onion/", 
        "http://dxcha4em3lwfsotttsftaeotqnyma43sa52qaapyspxk4f7payqogqid.onion/", 
        "http://loginzlib2vrak5zzpcocc3ouizykn6k5qecgj2tzlnab5wcbqhembyd.onion/", 
        "http://4usoivrpy52lmc4mgn2h34cmfiltslesthr56yttv2pxudd3dapqciyd.onion/", 
        "http://ciadotgov4sjwlzihbbgxnqg3xiyrg7so2r2o3lt5wz5ypk4sxyjstad.onion", 
        "http://articles24t2d47kb6rbabobokvrnymh2smkleosntcu6qxou6sxewyd.onion", 
        "http://isaqathsnae73abq7y3bxvhvkbnnpoygue4zhllfqozzy3l4zvgacqyd.onion/", 
        "http://r7lscyg5lncab4gm77ldshetqoutj3btq53aw44ixnz3iinc3oaz7qqd.onion/", 
        "http://tirxscsg3pcenlff67ecn2kb3jfv3ori7bgwryyn7btktohfdkms2cyd.onion/", 
        "http://s3p666he6q6djb6u3ekjdkmoyd77w63zq6gqf6sde54yg6bdfqukz2qd.onion", 
        "http://ransomocmou6mnbquqz44ewosbkjk3o5qjsl3orawojexfook2j7esad.onion/", 
        "http://ttauyzmy4kbm5yxpujpnahy7uxwnb32hh3dja7uda64vefpkomf3s4yd.onion/", 
        "http://nvidialkagnt37uon4hnwkz7xruhlpipeaz6j6zlugqf4mlpdfp6hgqd.onion", 
        "http://notbumpz34bgbz4yfdigxvd6vzwtxc3zpt5imukgl6bvip2nikdmdaad.onion/", 
        "http://ng27owmagn5amdm7l5s3rsqxwscl5ynppnis5dqcasogkyxcfqn7psid.onion/", 
        "http://tortorgohr62yxcizqpcpvwxupivwepkzl24cwkt4nnnkflvg7qraayd.onion/", 
        "http://uwovoq3luqdzqkuy5ynjl3lxh2gqbib2ceb77kcbr47ww4oyqyiahuid.onion", 
        "http://f7rkl2pb2uwyyrw3mgaxstwig3nhpl25fhjdvoultpn4j56n3vkbgkad.onion/", 
        "http://73yuwnoz7a36cdm2b6flpdkdkbimmjysmmqatyw6jeu4yy7gd6u4e3id.onion/", 
        "https://twitter3e4tixl4xyajtrzo62zg5vztmjuricljdp2c5kshju4avyoid.onion", 
        "http://briansclcfyc5oe34hgxnn3akr4hzshy3edpwxsilvbsojp2gwxr57qd.onion", 
        "http://kx5thpx2olielkihfyo4jgjqfb7zx7wxr3sd4xzt26ochei4m6f7tayd.onion/", 
        "http://vihnyh2wifmtfoa72xpufv4qjuf7xhlazot467jjlx657gwzazsr5byd.onion", 
        "http://kingz3mfshjqfij3pkjq2fkknjqb6dhdvctmfc6bnstla7ms6vjyjgid.onion", 
        "http://mute36pglo7mp5efzry5buqvn7dzlzhdbbksy7le3mz4ii4v6aganwid.onion/", 
        "http://zerobinftagjpeeebbvyzjcqyjpmjvynj5qlexwyxe7l3vqejxnqv5qd.onion", 
        "http://evilsiteagctwjyggb6ygnufkmr6bxnci4sf2xpjpgmxbjj72hdw5sid.onion/", 
        "http://redroomvovfxn5qd2yavwdhwb375cuoiqhhl24dkt73xma2zsdjiyyid.onion/", 
        "http://cnuiflerosm2uzh26tcuq5saapyto62nfajyhtqu37mwuzu2kyjbwrid.onion/", 
        "http://mempoolhqx4isw62xs7abwphsq7ldayuidyx2v2oethdhhj6mlo2r6ad.onion", 
        "http://bv3g7djr2lxggud4zp6uqikky4wueiep3nnz7xcdy5z6i4exau5z6jyd.onion/index.php", 
        "http://filesharehk7dfa4dcomiw36ycq54koe57cgwksksq3p7kxim4fozyid.onion/", 
        "http://moneyuu7hga6jpcfbamefsjwkv3bez3b3hkczpfzjb5zneunpqdh2uyd.onion", 
        "http://dnmxjaitaiafwmss2lx7tbs5bv66l7vjdmb5mtb3yqpxqhk3it5zivad.onion", 
        "http://darknetlidvrsli6iso7my54rjayjursyw637aypb6qambkoepmyq2yd.onion/", 
        "http://libraryfyuybp7oyidyya3ah5xvwgyx6weauoini7zyz555litmmumad.onion/", 
        "https://reddittorjg6rue252oqsxryoxengawnmo46qy4kyii5wtqnwfj4ooad.onion", 
        "http://cisland46psf56panbm2japxoxeguqekmataim54kn2ysucnk3yo7eyd.onion/", 
        "http://n2etofsu3q5cg6da44jzvaawiap7s3ougtuu4mmrahgizqgio3a4ssqd.onion/", 
        "http://zsxjtsgzborzdllyp64c6pwnjz5eic76bsksbxzqefzogwcydnkjy3yd.onion/", 
        "http://chatxxxocni24gsy5vuz52wzpiko4mxneujbbzp5hfsqz53cetk54eqd.onion/", 
        "http://retardidy7pvg5pu3vrum4lhc6w57ioeaudftc735xtgjlkse7elqnqd.onion/", 
        "http://thestock6nonb74owd6utzh4vld3xsf2n2fwxpwywjgq7maj47mvwmid.onion/", 
        "http://ao5zixnmkaydkt3rkxuvi5g6knyywreiu7lpqjv3hpjt7g2z675pzsid.onion/", 
        "http://darkarmy5dhdw5jbml2mitgpaorzdiubxxiq5tdzsqidz55kygsrz7id.onion/", 
        "http://pharmacy42hblqssvwizqvrwmy7oec5sfgjtr7hy6dup3t2lf2xzitqd.onion/", 
        "http://7sk2kov2xwx6cbc32phynrifegg6pklmzs7luwcggtzrnlsolxxuyfyd.onion/", 
        "http://bbzzzsvqcrqtki6umym6itiixfhni37ybtt7mkbjyxn2pgllzxf2qgyd.onion", 
        "http://breachdbsztfykg2fdaq2gnqnxfsbj5d35byz3yzj73hazydk4vq72qd.onion/", 
        "http://hell2ker5i3xsy6szrl2pulaqo3jhcz6pt7ffdxtuqjqiycvmlkcddqd.onion/", 
        "http://sy3tqe5tvukyk35kbqx4v3xjj5jxkrvrcdno4c4neg5kymz3zb5avzqd.onion/", 
        "http://applbwku7dfadkfkumiojsbxekuiafpr44idl7bxb2xll6neykvx35id.onion", 
        "http://4bujn7xhwvkmya7alh5fg7uiuw2keygaa5nx7f4so6r5o5whqsavq5yd.onion/", 
        "http://hidcorktriskdf6rz7xsgobe77a6mekyfjmc3byg5cpwvge32i5iqvad.onion/", 
        "http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion/", 
        "http://accountmwyiilytqztx6s45k5a6ud57x3gzmtumljheym5lqwelapaid.onion", 
        "http://hitman2xnfbcaaodfv6s2s5yagerqvb4cli2xeyoljhcf4yfvaa63ryd.onion/", 
        "http://u5qv3yvsykj673jlpyw7udpxablrcu4szuvlaee2eljudwemr2ddv3yd.onion/index.php", 
        "http://rfyb5tlhiqtiavwhikdlvb3fumxgqwtg2naanxtiqibidqlox5vispqd.onion/", 
        "http://j2dbibq43m4fdyry2pltez346mwneqfl7a5dzr2mgsouhp7lox2eu2qd.onion/", 
        "http://xssforumv3isucukbxhdhwz67hoa5e2voakcfkuieq4ch257vsburuid.onion", 
        "http://jnfqxx3pn7yur3xohy33cxuhjniluz2o5kdd4y5z373nzmk3dhqjzsad.onion/", 
        "http://34vnln24rlakgbk6gpityvljieayyw7q4bhdbbgs6zp2v5nbh345zgad.onion/", 
        "http://txxt.anonblogd4pcarck2ff6qlseyawjljaatp6wjq6rqpet2wfuoom42kyd.onion/", 
        "http://weeeedxejprore6lprzg5xwgkujwi27yk6vdj2qtizxoxm7dqe52vaid.onion", 
        "https://ho2hua2hfduv6f7hcbzdj2e6qdn4szgyy2jjnx545v4z3epq7uyrscid.onion", 
        "http://mxkbl4bdfc3764lkqwx4rxwengiq2s6odftz6h6p7l57eel7sdfwk3ad.onion/", 
        "https://danielas3rtn54uwmofdo3x2bsdifr47huasnmbgqzfrec5ubupvtpid.onion", 
        "http://pv7uymmi7dc2rmqmdyqkdohg4nnbyfxmuwz3csxpnymmgnlggurwpcad.onion/", 
        "http://dddirectinfv3htc4vl6mied5lpaatora7mmqkcf3sfjrx37fajigmyd.onion/", 
        "http://drc5nsfrndvr4zro5tpz3iwk5omrwpszq75upbxrmkfkpny5yrt4gfyd.onion/", 
        "http://bbzzzsvqcrqtki6umym6itiixfhni37ybtt7mkbjyxn2pgllzxf2qgyd.onion/", 
        "http://7eoz4h2nvw4zlr7gvlbutinqqpm546f5egswax54az6lt2u7e3t6d7yd.onion/", 
        "http://payplb3mm5bdkns6v7xou7xeefcl5bqedofcpnd462rw4gm4xbbwfpad.onion", 
        "http://onnimu5istptashw65gzzwu7lmqnelbffdcmrufm52h327he6te4ysqd.onion/", 
        "http://zqktlwiuavvvqqt4ybvgvi7tyo4hjl5xgfuvpdf6otjiycgwqbym2qad.onion/wiki/index.php/Main_Page", 
        "http://6hn4h63uroy22hbt5wn4e2a7ob2kfq6obwwgfxysgicwtukqiqrhzeyd.onion", 
        "http://jovotwwz7b7jihmy6hzwhohydfurzx3pf5hs4a7tdue6uufp2h6ae5yd.onion/", 
        "http://of3fshsmkrtrapgbsd5pueyqrydwir2yc7om44s2zjs3bjfomd232uqd.onion/", 
        "http://zkaan2xfbuxia2wpf7ofnkbz6r5zdbbvxbunvp5g2iebopbfc4iqmbad.onion/", 
        "http://s4k4ceiapwwgcm3mkb6e4diqecpo7kvdnfr5gg7sph7jjppqkvwwqtyd.onion/", 
        "http://us63bgjkxwpyrpvsqom6kw3jcy2yujbplkhtzt64yykt42ne2ms7p4yd.onion/", 
        "http://hackeoyrzjy3ob4cdr2q56bgp7cpatruphcxvgbfsiw6zeqcc36e4ryd.onion", 
        "http://rapeherfqriv56f4oudqxzqt55sadsofzltgjv4lfdbw4aallyp33vqd.onion/"]
     ;


    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let path = Path::new(filename);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);

    let mut to_visit: VecDeque<String> = VecDeque::new();

    for line in reader.lines() {
        match line {
            Ok(url) => to_visit.push_back(url),
            Err(why) => println!("couldn't read line: {}", why),
        }
    }


    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = Arc::clone(&stop_flag);
    thread::spawn(move || {
        let mut buffer = [0; 1];
        let stdin = io::stdin();
        loop {
            stdin.lock().read_exact(&mut buffer).unwrap();
            if buffer[0] == b'q' {
                println!("notified");
                stop_flag_clone.store(true, Ordering::SeqCst);
                break;
            }
        }
    });


    let mut crawler = crawler::Crawler::new::<500>(
        to_visit,
        stop_flag,
        Some("test.txt".to_string()),
    );

    let _ = crawler.start().await;

    Ok(())
}
