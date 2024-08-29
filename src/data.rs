use nalgebra::{ArrayStorage, SVector, SMatrix};

use crate::{obs::{ObserverTag, Observer}, spc::{Spectrum, Category, NS}};

pub static CIE1931: Observer = Observer{id: ObserverTag::Std1931, lumconst: 683.0, data: SMatrix::<f64, 3, NS>::from_array_storage( ArrayStorage([
    [0.001368, 0.000039, 0.006450001], [0.00150205, 0.0000428264, 0.007083216], [0.001642328, 0.0000469146, 0.007745488], [0.001802382, 0.0000515896, 0.008501152], [0.001995757, 0.0000571764, 0.009414544],
    [0.002236, 0.000064, 0.01054999], [0.002535385, 0.00007234421, 0.0119658], [0.002892603, 0.00008221224, 0.01365587], [0.003300829, 0.00009350816, 0.01558805], [0.003753236, 0.0001061361, 0.01773015],
    [0.004243, 0.00012, 0.02005001], [0.004762389, 0.000134984, 0.02251136], [0.005330048, 0.000151492, 0.02520288], [0.005978712, 0.000170208, 0.02827972], [0.006741117, 0.000191816, 0.03189704],
    [0.00765, 0.000217, 0.03621], [0.008751373, 0.0002469067, 0.04143771], [0.01002888, 0.00028124, 0.04750372], [0.0114217, 0.00031852, 0.05411988], [0.01286901, 0.0003572667, 0.06099803],
    [0.01431, 0.000396, 0.06785001], [0.01570443, 0.0004337147, 0.07448632], [0.01714744, 0.000473024, 0.08136156], [0.01878122, 0.000517876, 0.08915364], [0.02074801, 0.0005722187, 0.09854048],
    [0.02319, 0.00064, 0.1102], [0.02620736, 0.00072456, 0.1246133], [0.02978248, 0.0008255, 0.1417017], [0.03388092, 0.00094116, 0.1613035], [0.03846824, 0.00106988, 0.1832568],
    [0.04351, 0.00121, 0.2074], [0.0489956, 0.001362091, 0.2336921], [0.0550226, 0.001530752, 0.2626114], [0.0617188, 0.001720368, 0.2947746], [0.069212, 0.001935323, 0.3307985],
    [0.07763, 0.00218, 0.3713], [0.08695811, 0.0024548, 0.4162091], [0.09717672, 0.002764, 0.4654642], [0.1084063, 0.0031178, 0.5196948], [0.1207672, 0.0035264, 0.5795303],
    [0.13438, 0.004, 0.6456], [0.1493582, 0.00454624, 0.7184838], [0.1653957, 0.00515932, 0.7967133], [0.1819831, 0.00582928, 0.8778459], [0.198611, 0.00654616, 0.959439],
    [0.21477, 0.0073, 1.0390501], [0.2301868, 0.008086507, 1.1153673], [0.2448797, 0.00890872, 1.1884971], [0.2587773, 0.00976768, 1.2581233], [0.2718079, 0.01066443, 1.3239296],
    [0.2839, 0.0116, 1.3856], [0.2949438, 0.01257317, 1.4426352], [0.3048965, 0.01358272, 1.4948035], [0.3137873, 0.01462968, 1.5421903], [0.3216454, 0.01571509, 1.5848807],
    [0.3285, 0.01684, 1.62296], [0.3343513, 0.01800736, 1.6564048], [0.3392101, 0.01921448, 1.6852959], [0.3431213, 0.02045392, 1.7098745], [0.3461296, 0.02171824, 1.7303821],
    [0.34828, 0.023, 1.74706], [0.3495999, 0.02429461, 1.7600446], [0.3501474, 0.02561024, 1.7696233], [0.350013, 0.02695857, 1.7762637], [0.349287, 0.02835125, 1.7804334],
    [0.34806, 0.0298, 1.7826], [0.3463733, 0.03131083, 1.7829682], [0.3442624, 0.03288368, 1.7816998], [0.3418088, 0.03452112, 1.7791982], [0.3390941, 0.03622571, 1.7758671],
    [0.3362, 0.038, 1.77211], [0.3331977, 0.03984667, 1.7682589], [0.3300411, 0.041768, 1.764039], [0.3266357, 0.043766, 1.7589438], [0.3228868, 0.04584267, 1.7524663],
    [0.3187, 0.048, 1.7441], [0.3140251, 0.05024368, 1.7335595], [0.308884, 0.05257304, 1.7208581], [0.3032904, 0.05498056, 1.7059369], [0.2972579, 0.05745872, 1.6887372],
    [0.2908, 0.06, 1.6692], [0.2839701, 0.06260197, 1.6475287], [0.2767214, 0.06527752, 1.6234127], [0.2689178, 0.06804208, 1.5960223], [0.2604227, 0.07091109, 1.564528],
    [0.2511, 0.0739, 1.5281], [0.2408475, 0.077016, 1.4861114], [0.2298512, 0.0802664, 1.4395215], [0.2184072, 0.0836668, 1.3898799], [0.2068115, 0.0872328, 1.3387362],
    [0.19536, 0.09098, 1.28764], [0.1842136, 0.09491755, 1.2374223], [0.1733273, 0.09904584, 1.1878243], [0.1626881, 0.1033674, 1.1387611], [0.1522833, 0.1078846, 1.090148],
    [0.1421, 0.1126, 1.0419], [0.1321786, 0.117532, 0.9941976], [0.1225696, 0.1226744, 0.9473473], [0.1132752, 0.1279928, 0.9014531], [0.1042979, 0.1334528, 0.8566193],
    [0.09564, 0.13902, 0.8129501], [0.08729955, 0.1446764, 0.7705173], [0.07930804, 0.1504693, 0.7294448], [0.07171776, 0.1564619, 0.6899136], [0.06458099, 0.1627177, 0.6521049],
    [0.05795001, 0.1693, 0.6162], [0.05186211, 0.1762431, 0.5823286], [0.04628152, 0.1835581, 0.5504162], [0.04115088, 0.1912735, 0.5203376], [0.03641283, 0.199418, 0.4919673],
    [0.03201, 0.20802, 0.46518], [0.0279172, 0.2171199, 0.4399246], [0.0241444, 0.2267345, 0.4161836], [0.020687, 0.2368571, 0.3938822], [0.0175404, 0.2474812, 0.3729459],
    [0.0147, 0.2586, 0.3533], [0.01216179, 0.2701849, 0.3348578], [0.00991996, 0.2822939, 0.3175521], [0.00796724, 0.2950505, 0.3013375], [0.006296346, 0.308578, 0.2861686],
    [0.0049, 0.323, 0.272], [0.003777173, 0.3384021, 0.2588171], [0.00294532, 0.3546858, 0.2464838], [0.00242488, 0.3716986, 0.2347718], [0.002236293, 0.3892875, 0.2234533],
    [0.0024, 0.4073, 0.2123], [0.00292552, 0.4256299, 0.2011692], [0.00383656, 0.4443096, 0.1901196], [0.00517484, 0.4633944, 0.1792254], [0.00698208, 0.4829395, 0.1685608],
    [0.0093, 0.503, 0.1582], [0.01214949, 0.5235693, 0.1481383], [0.01553588, 0.544512, 0.1383758], [0.01947752, 0.56569, 0.1289942], [0.02399277, 0.5869653, 0.1200751],
    [0.0291, 0.6082, 0.1117], [0.03481485, 0.6293456, 0.1039048], [0.04112016, 0.6503068, 0.09666748], [0.04798504, 0.6708752, 0.08998272], [0.05537861, 0.6908424, 0.08384531],
    [0.06327, 0.71, 0.07824999], [0.07163501, 0.7281852, 0.07320899], [0.08046224, 0.7454636, 0.06867816], [0.08973996, 0.7619694, 0.06456784], [0.09945645, 0.7778368, 0.06078835],
    [0.1096, 0.7932, 0.05725001], [0.1201674, 0.8081104, 0.05390435], [0.1311145, 0.8224962, 0.05074664], [0.1423679, 0.8363068, 0.04775276], [0.1538542, 0.8494916, 0.04489859],
    [0.1655, 0.862, 0.04216], [0.1772571, 0.8738108, 0.03950728], [0.18914, 0.8849624, 0.03693564], [0.2011694, 0.8954936, 0.03445836], [0.2133658, 0.9054432, 0.03208872],
    [0.2257499, 0.9148501, 0.02984], [0.2383209, 0.9237348, 0.02771181], [0.2510668, 0.9320924, 0.02569444], [0.2639922, 0.9399226, 0.02378716], [0.2771017, 0.9472252, 0.02198925],
    [0.2904, 0.954, 0.0203], [0.3038912, 0.9602561, 0.01871805], [0.3175726, 0.9660074, 0.01724036], [0.3314384, 0.9712606, 0.01586364], [0.3454828, 0.9760225, 0.01458461],
    [0.3597, 0.9803, 0.0134], [0.3740839, 0.9840924, 0.01230723], [0.3886396, 0.9874182, 0.01130188], [0.4033784, 0.9903128, 0.01037792], [0.4183115, 0.9928116, 0.009529306],
    [0.4334499, 0.9949501, 0.008749999], [0.4487953, 0.9967108, 0.0080352], [0.464336, 0.9980983, 0.0073816], [0.480064, 0.999112, 0.0067854], [0.4959713, 0.9997482, 0.0062428],
    [0.5120501, 1.0, 0.005749999], [0.5282959, 0.9998567, 0.0053036], [0.5446916, 0.9993046, 0.0048998], [0.5612094, 0.9983255, 0.0045342], [0.5778215, 0.9968987, 0.0042024],
    [0.5945, 0.995, 0.0039], [0.6112209, 0.9926005, 0.0036232], [0.6279758, 0.9897426, 0.0033706], [0.6447602, 0.9864444, 0.0031414], [0.6615697, 0.9827241, 0.0029348],
    [0.6784, 0.9786, 0.002749999], [0.6952392, 0.9740837, 0.0025852], [0.7120586, 0.9691712, 0.0024386], [0.7288284, 0.9638568, 0.0023094], [0.7455188, 0.9581349, 0.0021968],
    [0.7621, 0.952, 0.0021], [0.7785432, 0.9454504, 0.002017733], [0.7948256, 0.9384992, 0.0019482], [0.8109264, 0.9311628, 0.0018898], [0.8268248, 0.9234576, 0.001840933],
    [0.8425, 0.9154, 0.0018], [0.8579325, 0.9070064, 0.001766267], [0.8730816, 0.8982772, 0.0017378], [0.8878944, 0.8892048, 0.0017112], [0.9023181, 0.8797816, 0.001683067],
    [0.9163, 0.87, 0.001650001], [0.9297995, 0.8598613, 0.001610133], [0.9427984, 0.849392, 0.0015644], [0.9552776, 0.838622, 0.0015136], [0.9672179, 0.8275813, 0.001458533],
    [0.9786, 0.8163, 0.0014], [0.9893856, 0.8047947, 0.001336667], [0.9995488, 0.793082, 0.00127], [1.0090892, 0.781192, 0.001205], [1.0180064, 0.7691547, 0.001146667],
    [1.0263, 0.757, 0.0011], [1.0339827, 0.7447541, 0.0010688], [1.040986, 0.7324224, 0.0010494], [1.047188, 0.7200036, 0.0010356], [1.0524667, 0.7074965, 0.0010212],
    [1.0567, 0.6949, 0.001], [1.0597944, 0.6822192, 0.00096864], [1.0617992, 0.6694716, 0.00092992], [1.0628068, 0.6566744, 0.00088688], [1.0629096, 0.6438448, 0.00084256],
    [1.0622, 0.631, 0.0008], [1.0607352, 0.6181555, 0.00076096], [1.0584436, 0.6053144, 0.00072368], [1.0552244, 0.5924756, 0.00068592], [1.0509768, 0.5796379, 0.00064544],
    [1.0456, 0.5668, 0.0006], [1.0390369, 0.5539611, 0.0005478667], [1.0313608, 0.5411372, 0.0004916], [1.0226662, 0.5283528, 0.0004354], [1.0130477, 0.5156323, 0.0003834667],
    [1.0026, 0.503, 0.00034], [0.9913675, 0.4904688, 0.0003072533], [0.9793314, 0.4780304, 0.00028316], [0.9664916, 0.4656776, 0.00026544], [0.9528479, 0.4534032, 0.0002518133],
    [0.9384, 0.4412, 0.00024], [0.923194, 0.42908, 0.0002295467], [0.907244, 0.417036, 0.00022064], [0.890502, 0.405032, 0.00021196], [0.87292, 0.393032, 0.0002021867],
    [0.8544499, 0.381, 0.00019], [0.835084, 0.3689184, 0.0001742133], [0.814946, 0.3568272, 0.00015564], [0.794186, 0.3447768, 0.00013596], [0.772954, 0.3328176, 0.0001168533],
    [0.7514, 0.321, 0.0001], [0.7295836, 0.3093381, 0.00008613333], [0.7075888, 0.2978504, 0.0000746], [0.6856022, 0.2865936, 0.000065], [0.6638104, 0.2756245, 0.00005693333],
    [0.6424, 0.265, 0.00004999999], [0.6215149, 0.2547632, 0.00004416], [0.6011138, 0.2448896, 0.00003948], [0.5811052, 0.2353344, 0.00003572], [0.5613977, 0.2260528, 0.00003264],
    [0.5419, 0.217, 0.00003], [0.5225995, 0.2081616, 0.00002765333], [0.5035464, 0.1995488, 0.00002556], [0.4847436, 0.1911552, 0.00002364], [0.4661939, 0.1829744, 0.00002181333],
    [0.4479, 0.175, 0.00002], [0.4298613, 0.1672235, 0.00001813333], [0.412098, 0.1596464, 0.0000162], [0.394644, 0.1522776, 0.0000142], [0.3775333, 0.1451259, 0.00001213333],
    [0.3608, 0.1382, 0.00001], [0.3444563, 0.1315003, 0.000007733333], [0.3285168, 0.1250248, 0.0000054], [0.3130192, 0.1187792, 0.0000032], [0.2980011, 0.1127691, 0.000001333333],
    [0.2835, 0.107, 0.0], [0.2695448, 0.1014762, 0.0], [0.2561184, 0.09618864, 0.0], [0.2431896, 0.09112296, 0.0], [0.2307272, 0.08626485, 0.0],
    [0.2187, 0.0816, 0.0], [0.2070971, 0.07712064, 0.0], [0.1959232, 0.07282552, 0.0], [0.1851708, 0.06871008, 0.0], [0.1748323, 0.06476976, 0.0],
    [0.1649, 0.061, 0.0], [0.1553667, 0.05739621, 0.0], [0.14623, 0.05395504, 0.0], [0.13749, 0.05067376, 0.0], [0.1291467, 0.04754965, 0.0],
    [0.1212, 0.04458, 0.0], [0.1136397, 0.04175872, 0.0], [0.106465, 0.03908496, 0.0], [0.09969044, 0.03656384, 0.0], [0.09333061, 0.03420048, 0.0],
    [0.0874, 0.032, 0.0], [0.08190096, 0.02996261, 0.0], [0.07680428, 0.02807664, 0.0], [0.07207712, 0.02632936, 0.0], [0.06768664, 0.02470805, 0.0],
    [0.0636, 0.0232, 0.0], [0.05980685, 0.02180077, 0.0], [0.05628216, 0.02050112, 0.0], [0.05297104, 0.01928108, 0.0], [0.04981861, 0.01812069, 0.0],
    [0.04677, 0.017, 0.0], [0.04378405, 0.01590379, 0.0], [0.04087536, 0.01483718, 0.0], [0.03807264, 0.01381068, 0.0], [0.03540461, 0.01283478, 0.0],
    [0.0329, 0.01192, 0.0], [0.03056419, 0.01106831, 0.0], [0.02838056, 0.01027339, 0.0], [0.02634484, 0.009533311, 0.0], [0.02445275, 0.008846157, 0.0],
    [0.0227, 0.00821, 0.0], [0.02108429, 0.007623781, 0.0], [0.01959988, 0.007085424, 0.0], [0.01823732, 0.006591476, 0.0], [0.01698717, 0.006138485, 0.0],
    [0.01584, 0.005723, 0.0], [0.01479064, 0.005343059, 0.0], [0.01383132, 0.004995796, 0.0], [0.01294868, 0.004676404, 0.0], [0.0121292, 0.004380075, 0.0],
    [0.01135916, 0.004102, 0.0], [0.01062935, 0.003838453, 0.0], [0.009938846, 0.003589099, 0.0], [0.009288422, 0.003354219, 0.0], [0.008678854, 0.003134093, 0.0],
    [0.008110916, 0.002929, 0.0], [0.007582388, 0.002738139, 0.0], [0.007088746, 0.002559876, 0.0], [0.006627313, 0.002393244, 0.0], [0.006195408, 0.002237275, 0.0],
    [0.005790346, 0.002091, 0.0], [0.005409826, 0.001953587, 0.0], [0.005052583, 0.00182458, 0.0], [0.004717512, 0.00170358, 0.0], [0.004403507, 0.001590187, 0.0],
    [0.004109457, 0.001484, 0.0], [0.003833913, 0.001384496, 0.0], [0.003575748, 0.001291268, 0.0], [0.003334342, 0.001204092, 0.0], [0.003109075, 0.001122744, 0.0],
    [0.002899327, 0.001047, 0.0], [0.002704348, 0.0009765896, 0.0], [0.00252302, 0.0009111088, 0.0], [0.002354168, 0.0008501332, 0.0], [0.002196616, 0.0007932384, 0.0],
    [0.00204919, 0.00074, 0.0], [0.00191096, 0.0006900827, 0.0], [0.001781438, 0.00064331, 0.0], [0.00166011, 0.000599496, 0.0], [0.001546459, 0.0005584547, 0.0],
    [0.001439971, 0.00052, 0.0], [0.001340042, 0.0004839136, 0.0], [0.001246275, 0.0004500528, 0.0], [0.001158471, 0.0004183452, 0.0], [0.00107643, 0.0003887184, 0.0],
    [0.0009999493, 0.0003611, 0.0], [0.0009287358, 0.0003353835, 0.0], [0.0008624332, 0.0003114404, 0.0], [0.0008007503, 0.0002891656, 0.0], [0.000743396, 0.0002684539, 0.0],
    [0.0006900786, 0.0002492, 0.0], [0.0006405156, 0.0002313019, 0.0], [0.0005945021, 0.0002146856, 0.0], [0.0005518646, 0.0001992884, 0.0], [0.000512429, 0.0001850475, 0.0],
    [0.0004760213, 0.0001719, 0.0], [0.0004424536, 0.0001597781, 0.0], [0.0004115117, 0.0001486044, 0.0], [0.0003829814, 0.0001383016, 0.0], [0.0003566491, 0.0001287925, 0.0],
    [0.0003323011, 0.00012, 0.0], [0.0003097586, 0.0001118595, 0.0], [0.0002888871, 0.0001043224, 0.0], [0.0002695394, 0.0000973356, 0.0], [0.0002515682, 0.00009084587, 0.0],
    [0.0002348261, 0.0000848, 0.0], [0.000219171, 0.00007914667, 0.0], [0.0002045258, 0.000073858, 0.0], [0.0001908405, 0.000068916, 0.0], [0.0001780654, 0.00006430267, 0.0],
    [0.0001661505, 0.00006, 0.0], [0.0001550236, 0.00005598187, 0.0], [0.0001446219, 0.0000522256, 0.0], [0.0001349098, 0.0000487184, 0.0], [0.000125852, 0.00004544747, 0.0],
    [0.000117413, 0.0000424, 0.0], [0.0001095515, 0.00003956104, 0.0], [0.0001022245, 0.00003691512, 0.0], [0.00009539445, 0.00003444868, 0.0], [0.0000890239, 0.00003214816, 0.0],
    [0.00008307527, 0.00003, 0.0], [0.00007751269, 0.00002799125, 0.0], [0.00007231304, 0.00002611356, 0.0], [0.00006745778, 0.00002436024, 0.0], [0.00006292844, 0.00002272461, 0.0],
    [0.00005870652, 0.0000212, 0.0], [0.00005477028, 0.00001977855, 0.0], [0.00005109918, 0.00001845285, 0.0], [0.00004767654, 0.00001721687, 0.0], [0.00004448567, 0.00001606459, 0.0],
    [0.00004150994, 0.00001499, 0.0]
]))};

/// D65 CIE Standard Illuminant.
///
/// Data from https://cie.co.at/datatable/cie-standard-illuminant-d65, truncated
/// to a range from 380 to 780 nanometer.  These data, using the `CIE1931` color
/// matching functions, results in a slightly different set of chromaticity
/// values then published historically, which are calculated with a larger domain, and
/// using a step size of 5 nanometer.  As this library uses spectral
/// distributions to allow the use of different observers, and follows the
/// `CIE015:2004` Colorimetry standard, we accept this deviation.
pub static D65: Spectrum = Spectrum {
    cat: Category::Illuminant,
    total: None,
    data: SVector::<f64, NS>::from_array_storage(ArrayStorage([[
        49.9755, 50.4428, 50.91, 51.3773, 51.8446, 52.3118, 52.7791, 53.2464, 53.7137, 54.1809, 54.6482, 57.4589, 60.2695, 63.0802, 65.8909, 68.7015, 71.5122, 74.3229, 77.1336, 79.9442, 
        82.7549, 83.628, 84.5011, 85.3742, 86.2473, 87.1204, 87.9936, 88.8667, 89.7398, 90.6129, 91.486, 91.6806, 91.8752, 92.0697, 92.2643, 92.4589, 92.6535, 92.8481, 93.0426, 93.2372, 
        93.4318, 92.7568, 92.0819, 91.4069, 90.732, 90.057, 89.3821, 88.7071, 88.0322, 87.3572, 86.6823, 88.5006, 90.3188, 92.1371, 93.9554, 95.7736, 97.5919, 99.4102, 101.228, 103.047, 
        104.865, 106.079, 107.294, 108.508, 109.722, 110.936, 112.151, 113.365, 114.579, 115.794, 117.008, 117.088, 117.169, 117.249, 117.33, 117.41, 117.49, 117.571, 117.651, 117.732, 
        117.812, 117.517, 117.222, 116.927, 116.632, 116.336, 116.041, 115.746, 115.451, 115.156, 114.861, 114.967, 115.073, 115.18, 115.286, 115.392, 115.498, 115.604, 115.711, 115.817, 
        115.923, 115.212, 114.501, 113.789, 113.078, 112.367, 111.656, 110.945, 110.233, 109.522, 108.811, 108.865, 108.92, 108.974, 109.028, 109.082, 109.137, 109.191, 109.245, 109.3, 
        109.354, 109.199, 109.044, 108.888, 108.733, 108.578, 108.423, 108.268, 108.112, 107.957, 107.802, 107.501, 107.2, 106.898, 106.597, 106.296, 105.995, 105.694, 105.392, 105.091, 
        104.79, 105.08, 105.37, 105.66, 105.95, 106.239, 106.529, 106.819, 107.109, 107.399, 107.689, 107.361, 107.032, 106.704, 106.375, 106.047, 105.719, 105.39, 105.062, 104.733, 
        104.405, 104.369, 104.333, 104.297, 104.261, 104.225, 104.19, 104.154, 104.118, 104.082, 104.046, 103.641, 103.237, 102.832, 102.428, 102.023, 101.618, 101.214, 100.809, 100.405, 
        100.0, 99.6334, 99.2668, 98.9003, 98.5337, 98.1671, 97.8005, 97.4339, 97.0674, 96.7008, 96.3342, 96.2796, 96.225, 96.1703, 96.1157, 96.0611, 96.0065, 95.9519, 95.8972, 95.8426, 
        95.788, 95.0778, 94.3675, 93.6573, 92.947, 92.2368, 91.5266, 90.8163, 90.1061, 89.3958, 88.6856, 88.8177, 88.9497, 89.0818, 89.2138, 89.3459, 89.478, 89.61, 89.7421, 89.8741, 
        90.0062, 89.9655, 89.9248, 89.8841, 89.8434, 89.8026, 89.7619, 89.7212, 89.6805, 89.6398, 89.5991, 89.4091, 89.219, 89.029, 88.8389, 88.6489, 88.4589, 88.2688, 88.0788, 87.8887, 
        87.6987, 87.2577, 86.8167, 86.3757, 85.9347, 85.4936, 85.0526, 84.6116, 84.1706, 83.7296, 83.2886, 83.3297, 83.3707, 83.4118, 83.4528, 83.4939, 83.535, 83.576, 83.6171, 83.6581, 
        83.6992, 83.332, 82.9647, 82.5975, 82.2302, 81.863, 81.4958, 81.1285, 80.7613, 80.394, 80.0268, 80.0456, 80.0644, 80.0831, 80.1019, 80.1207, 80.1395, 80.1583, 80.177, 80.1958, 
        80.2146, 80.4209, 80.6272, 80.8336, 81.0399, 81.2462, 81.4525, 81.6588, 81.8652, 82.0715, 82.2778, 81.8784, 81.4791, 81.0797, 80.6804, 80.281, 79.8816, 79.4823, 79.0829, 78.6836, 
        78.2842, 77.4279, 76.5716, 75.7153, 74.859, 74.0027, 73.1465, 72.2902, 71.4339, 70.5776, 69.7213, 69.9101, 70.0989, 70.2876, 70.4764, 70.6652, 70.854, 71.0428, 71.2315, 71.4203, 
        71.6091, 71.8831, 72.1571, 72.4311, 72.7051, 72.979, 73.253, 73.527, 73.801, 74.075, 74.349, 73.0745, 71.8, 70.5255, 69.251, 67.9765, 66.702, 65.4275, 64.153, 62.8785, 
        61.604, 62.4322, 63.2603, 64.0885, 64.9166, 65.7448, 66.573, 67.4011, 68.2293, 69.0574, 69.8856, 70.4057, 70.9259, 71.446, 71.9662, 72.4863, 73.0064, 73.5266, 74.0467, 74.5669, 
        75.087, 73.9376, 72.7881, 71.6387, 70.4893, 69.3398, 68.1904, 67.041, 65.8916, 64.7421, 63.5927, 61.8752, 60.1578, 58.4403, 56.7229, 55.0054, 53.288, 51.5705, 49.8531, 48.1356, 
        46.4182, 48.4569, 50.4956, 52.5344, 54.5731, 56.6118, 58.6505, 60.6892, 62.728, 64.7667, 66.8054, 66.4631, 66.1209, 65.7786, 65.4364, 65.0941, 64.7518, 64.4096, 64.0673, 63.7251, 
        63.3828
    ]]))
};

/// D50 CIE standard Illuminant.
///
/// Data from https://cie.co.at/datatable/cie-standard-illuminant-d50, truncated
/// to a range from 380 to 780 nanometer.  These data, using the `CIE1931` color
/// matching functions, results in a slightly different set of chromaticity
/// values then published historically, which are calculated with a larger domain, and
/// using a step size of 5 nanometer.  As this library uses spectral
/// distributions to allow the use of different observers, and follows the
/// `CIE015:2004` Colorimetry standard, we accept this deviation.
pub static D50: Spectrum = Spectrum {
    cat: Category::Illuminant,
    total: None,
    data: SVector::<f64, NS>::from_array_storage(ArrayStorage([[
        24.4875, 25.0258, 25.5641, 26.1024, 26.6407, 27.179, 27.7174, 28.2557, 28.794, 29.3323, 29.8706, 31.8144, 33.7581, 35.7018, 37.6456, 39.5894, 41.5331, 43.4768,
        45.4206, 47.3644, 49.3081, 50.0286, 50.749, 51.4695, 52.19, 52.9104, 53.6309, 54.3514, 55.0719, 55.7923, 56.5128, 56.8649, 57.217, 57.5691, 57.9212, 58.2733,
        58.6254, 58.9775, 59.3296, 59.6817, 60.0338, 59.8122, 59.5905, 59.3689, 59.1473, 58.9256, 58.704, 58.4824, 58.2608, 58.0391, 57.8175, 59.5182, 61.219, 62.9197,
        64.6205, 66.3212, 68.0219, 69.7227, 71.4234, 73.1242, 74.8249, 76.0671, 77.3094, 78.5516, 79.7938, 81.036, 82.2783, 83.5205, 84.7627, 86.005, 87.2472, 87.5837,
        87.9202, 88.2567, 88.5932, 88.9297, 89.2662, 89.6027, 89.9392, 90.2757, 90.6122, 90.6878, 90.7634, 90.839, 90.9146, 90.9902, 91.0657, 91.1413, 91.2169, 91.2925,
        91.3681, 91.7421, 92.1162, 92.4902, 92.8643, 93.2383, 93.6123, 93.9864, 94.3604, 94.7345, 95.1085, 94.7939, 94.4793, 94.1648, 93.8502, 93.5356, 93.221, 92.9064,
        92.5919, 92.2773, 91.9627, 92.3388, 92.7149, 93.091, 93.4671, 93.8432, 94.2193, 94.5954, 94.9715, 95.3476, 95.7237, 95.8127, 95.9016, 95.9906, 96.0795, 96.1685,
        96.2575, 96.3464, 96.4354, 96.5243, 96.6133, 96.6649, 96.7164, 96.768, 96.8196, 96.8712, 96.9227, 96.9743, 97.0259, 97.0774, 97.129, 97.626, 98.123, 98.62,
        99.117, 99.614, 100.111, 100.608, 101.105, 101.602, 102.099, 101.965, 101.83, 101.696, 101.561, 101.427, 101.292, 101.158, 101.024, 100.889, 100.755, 100.911,
        101.067, 101.223, 101.38, 101.536, 101.692, 101.848, 102.005, 102.161, 102.317, 102.085, 101.854, 101.622, 101.39, 101.158, 100.927, 100.695, 100.463, 100.232,
        100.0, 99.7735, 99.547, 99.3205, 99.094, 98.8675, 98.641, 98.4145, 98.188, 97.9615, 97.735, 97.8533, 97.9716, 98.0899, 98.2082, 98.3265, 98.4448, 98.5631,
        98.6814, 98.7997, 98.918, 98.3761, 97.8342, 97.2922, 96.7503, 96.2084, 95.6665, 95.1246, 94.5826, 94.0407, 93.4988, 93.9177, 94.3366, 94.7555, 95.1744, 95.5933,
        96.0122, 96.4311, 96.85, 97.2689, 97.6878, 97.8459, 98.0041, 98.1622, 98.3203, 98.4784, 98.6366, 98.7947, 98.9528, 99.111, 99.2691, 99.2463, 99.2236, 99.2008,
        99.1781, 99.1553, 99.1325, 99.1098, 99.087, 99.0643, 99.0415, 98.7095, 98.3776, 98.0456, 97.7136, 97.3816, 97.0497, 96.7177, 96.3857, 96.0538, 95.7218, 96.0353,
        96.3489, 96.6624, 96.976, 97.2895, 97.603, 97.9166, 98.2301, 98.5437, 98.8572, 98.5382, 98.2192, 97.9002, 97.5812, 97.2622, 96.9432, 96.6242, 96.3052, 95.9862,
        95.6672, 95.9195, 96.1717, 96.424, 96.6762, 96.9285, 97.1808, 97.433, 97.6853, 97.9375, 98.1898, 98.6712, 99.1525, 99.6339, 100.115, 100.597, 101.078, 101.559,
        102.041, 102.522, 103.003, 102.616, 102.229, 101.842, 101.455, 101.068, 100.681, 100.294, 99.9071, 99.52, 99.133, 97.9578, 96.7826, 95.6074, 94.4322, 93.257,
        92.0817, 90.9065, 89.7313, 88.5561, 87.3809, 87.8032, 88.2254, 88.6477, 89.0699, 89.4922, 89.9145, 90.3367, 90.759, 91.1812, 91.6035, 91.732, 91.8605, 91.989,
        92.1175, 92.246, 92.3746, 92.5031, 92.6316, 92.7601, 92.8886, 91.2852, 89.6818, 88.0783, 86.4749, 84.8715, 83.2681, 81.6647, 80.0612, 78.4578, 76.8544, 77.8201,
        78.7858, 79.7514, 80.7171, 81.6828, 82.6485, 83.6142, 84.5798, 85.5455, 86.5112, 87.1181, 87.7249, 88.3318, 88.9386, 89.5455, 90.1524, 90.7592, 91.3661,
        91.9729, 92.5798, 91.1448, 89.7098, 88.2748, 86.8398, 85.4048, 83.9699, 82.5349, 81.0999, 79.6649, 78.2299, 76.1761, 74.1223, 72.0685, 70.0147, 67.9608, 65.907,
        63.8532, 61.7994, 59.7456, 57.6918, 60.2149, 62.738, 65.2612, 67.7843, 70.3074, 72.8305, 75.3536, 77.8768, 80.3999, 82.923, 82.4581, 81.9932, 81.5283, 81.0634,
        80.5985, 80.1336, 79.6687, 79.2038, 78.7389, 78.274
    ]]))
};

/// CIE A Standard Illuminant.
/// Data from https://cie.co.at/datatable/cie-standard-illuminant-d50, with DOI
/// https://doi.org/10.25039/CIE.DS.8jsxjrsn truncated to a range from 380 to
/// 780 nanometer.
pub static A: Spectrum = Spectrum {
    cat: Category::Illuminant,
    total: None,
    data: SVector::<f64, NS>::from_array_storage(ArrayStorage([[
        9.7951, 10.0096, 10.2273, 10.4481, 10.6722, 10.8996, 11.1302, 11.364, 11.6012, 11.8416, 12.0853,
        12.3324, 12.5828, 12.8366, 13.0938, 13.3543, 13.6182, 13.8855, 14.1563, 14.4304, 14.708, 14.9891,
        15.2736, 15.5616, 15.853, 16.148, 16.4464, 16.7484, 17.0538, 17.3628, 17.6753, 17.9913, 18.3108,
        18.6339, 18.9605, 19.2907, 19.6244, 19.9617, 20.3026, 20.647, 20.995, 21.3465, 21.7016, 22.0603,
        22.4225, 22.7883, 23.1577, 23.5307, 23.9072, 24.2873, 24.6709, 25.0581, 25.4489, 25.8432, 26.2411,
        26.6425, 27.0475, 27.456, 27.8681, 28.2836, 28.7027, 29.1253, 29.5515, 29.9811, 30.4142, 30.8508,
        31.2909, 31.7345, 32.1815, 32.632, 33.0859, 33.5432, 34.004, 34.4682, 34.9358, 35.4068, 35.8811,
        36.3588, 36.8399, 37.3243, 37.8121, 38.3031, 38.7975, 39.2951, 39.796, 40.3002, 40.8076, 41.3182,
        41.832, 42.3491, 42.8693, 43.3926, 43.9192, 44.4488, 44.9816, 45.5174, 46.0563, 46.5983, 47.1433,
        47.6913, 48.2423, 48.7963, 49.3533, 49.9132, 50.476, 51.0418, 51.6104, 52.1818, 52.7561, 53.3332,
        53.9132, 54.4958, 55.0813, 55.6694, 56.2603, 56.8539, 57.4501, 58.0489, 58.6504, 59.2545, 59.8611,
        60.4703, 61.082, 61.6962, 62.3128, 62.932, 63.5535, 64.1775, 64.8038, 65.4325, 66.0635, 66.6968,
        67.3324, 67.9702, 68.6102, 69.2525, 69.8969, 70.5435, 71.1922, 71.843, 72.4959, 73.1508, 73.8077,
        74.4666, 75.1275, 75.7903, 76.4551, 77.1217, 77.7902, 78.4605, 79.1326, 79.8065, 80.4821, 81.1595,
        81.8386, 82.5193, 83.2017, 83.8856, 84.5712, 85.2584, 85.947, 86.6372, 87.3288, 88.0219, 88.7165,
        89.4124, 90.1097, 90.8083, 91.5082, 92.2095, 92.912, 93.6157, 94.3206, 95.0267, 95.7339, 96.4423,
        97.1518, 97.8623, 98.5739, 99.2864, 100.0, 100.715, 101.43, 102.146, 102.864, 103.582, 104.301,
        105.02, 105.741, 106.462, 107.184, 107.906, 108.63, 109.354, 110.078, 110.803, 111.529, 112.255,
        112.982, 113.709, 114.436, 115.164, 115.893, 116.622, 117.351, 118.08, 118.81, 119.54, 120.27,
        121.001, 121.731, 122.462, 123.193, 123.924, 124.655, 125.386, 126.118, 126.849, 127.58, 128.312,
        129.043, 129.774, 130.505, 131.236, 131.966, 132.697, 133.427, 134.157, 134.887, 135.617, 136.346,
        137.075, 137.804, 138.532, 139.26, 139.988, 140.715, 141.441, 142.167, 142.893, 143.618, 144.343,
        145.067, 145.79, 146.513, 147.235, 147.957, 148.678, 149.398, 150.117, 150.836, 151.554, 152.271,
        152.988, 153.704, 154.418, 155.132, 155.845, 156.558, 157.269, 157.979, 158.689, 159.397, 160.104,
        160.811, 161.516, 162.221, 162.924, 163.626, 164.327, 165.028, 165.726, 166.424, 167.121, 167.816,
        168.51, 169.203, 169.895, 170.586, 171.275, 171.963, 172.65, 173.335, 174.019, 174.702, 175.383,
        176.063, 176.741, 177.419, 178.094, 178.769, 179.441, 180.113, 180.783, 181.451, 182.118, 182.783,
        183.447, 184.109, 184.77, 185.429, 186.087, 186.743, 187.397, 188.05, 188.701, 189.35, 189.998,
        190.644, 191.288, 191.931, 192.572, 193.211, 193.849, 194.484, 195.118, 195.75, 196.381, 197.009,
        197.636, 198.261, 198.884, 199.506, 200.125, 200.743, 201.359, 201.972, 202.584, 203.195, 203.803,
        204.409, 205.013, 205.616, 206.216, 206.815, 207.411, 208.006, 208.599, 209.189, 209.778, 210.365,
        210.949, 211.532, 212.112, 212.691, 213.268, 213.842, 214.415, 214.985, 215.553, 216.12, 216.684,
        217.246, 217.806, 218.364, 218.92, 219.473, 220.025, 220.574, 221.122, 221.667, 222.21, 222.751,
        223.29, 223.826, 224.361, 224.893, 225.423, 225.951, 226.477, 227.0, 227.522, 228.041, 228.558,
        229.073, 229.585, 230.096, 230.604, 231.11, 231.614, 232.115, 232.615, 233.112, 233.606, 234.099,
        234.589, 235.078, 235.564, 236.047, 236.529, 237.008, 237.485, 237.959, 238.432, 238.902, 239.37,
        239.836, 240.299, 240.76, 241.219, 241.675, 
    ]]))
};