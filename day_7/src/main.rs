use std::{collections::HashMap, cmp::Ordering};

fn main() {
    let input = "7KAK7 63
9889J 846
J37QA 670
T2T2J 5
827K6 552
KTT5T 116
JTJ63 457
55656 482
A924K 555
323QQ 568
2QQ5J 32
J4434 784
9T5A4 738
T666J 983
6K6J2 351
933TT 851
95567 364
5T598 858
55TT4 895
92T2T 893
2638T 17
TT3T9 509
8T954 579
834JQ 962
A9TK7 710
22552 201
T7627 879
K3795 875
J79A7 865
J9A99 986
56674 627
3QT25 470
53553 542
3T3K9 718
AA4AJ 345
95999 475
K3335 221
3QQQ3 207
TAAAA 152
A3AQ9 796
7Q922 53
JQJQQ 873
J888J 808
TJ223 828
52355 782
72889 264
4KA44 244
QJK6K 964
A3T76 71
38A27 303
5893T 841
576J4 826
884JQ 386
27233 359
45A55 906
K7KT6 677
T3KT3 254
7K663 291
78887 999
AT8JA 609
9A2T2 334
J3339 607
J99K9 427
K2K66 909
TT3J3 693
65732 67
5Q397 381
QKJA4 276
66678 582
3553K 171
2AJT7 631
8JKKK 78
44764 115
K3K33 367
46A66 407
94673 646
9QA53 597
JQ33Q 213
8A8J8 611
7QQ77 426
AJ99A 312
77883 103
6666J 914
33365 495
66565 890
88899 258
333A3 410
8KT8K 463
85888 241
8TQA2 151
666Q3 118
Q684T 820
82228 641
5335J 644
2Q7K6 806
47747 538
JTATA 186
74979 127
79TK9 142
78787 764
7QJTJ 650
A2228 880
24J4A 195
6Q3T3 358
5QJQT 286
7Q7QQ 232
J9JA9 145
39334 383
94495 155
K26K2 344
T78QQ 76
K292K 318
7AJAA 562
9875Q 876
Q5555 82
AT4TT 569
77779 671
QTTT2 994
7T64Q 651
74779 20
88T48 639
93999 373
3KJQK 385
T73TJ 612
766K8 60
TA538 161
542T5 52
K299T 249
T3A2A 981
92947 908
866J2 21
J7K23 149
JQ747 417
K7J77 837
AAA6A 925
7TJ77 506
K8QKK 248
KAAAJ 931
QQQ8Q 37
79972 961
777AA 56
722J2 752
64346 518
5JTJ2 629
K9999 898
T2AQK 256
66JJ6 222
A7A3A 557
877TT 247
TKJ59 608
53775 968
T8TTT 136
28862 691
37QTT 252
KJ442 772
866J6 604
324K5 596
T7973 309
2Q922 859
J4Q7Q 234
KQ39T 355
Q3Q35 635
TJ428 498
AKJ48 877
Q3Q7J 183
6K6KK 237
A5Q7K 775
8T22T 257
8AA88 225
777K4 523
QJK78 917
K5J82 788
Q6JAA 339
T3J63 483
5555A 3
7A333 31
J7QQ7 711
54QKQ 726
J463A 363
JJJJJ 905
3T33T 702
Q626Q 593
KQ9KJ 855
299A2 421
QK57Q 868
K29T3 570
88338 836
66696 637
5A755 24
T49T7 400
6QJK6 366
J3J43 33
65J66 701
8J982 821
2T68J 919
4J44K 209
97662 633
7J344 135
JK99K 269
4547A 49
6557Q 753
85585 395
JT555 465
88884 800
95A9J 980
44J46 429
8AJ8A 372
K42Q8 184
79863 329
QQJQ3 203
78A8K 166
6Q7Q6 47
87727 87
55755 148
99739 104
K956J 357
53533 10
54AQ3 761
66339 143
38833 563
AAAA2 304
6KATK 768
A6J83 101
626J6 165
JTJJQ 193
K7444 949
95432 420
TTT96 311
833AA 105
38233 439
2777Q 912
J6442 150
56A3K 371
JT24T 226
4KK88 532
2J972 823
TA4Q7 945
77TJT 578
55T9A 683
KJT3Q 394
TT9TT 28
T6A9A 932
32466 306
2A5J6 270
8A863 915
6J4AA 565
3AJ82 260
98789 96
6666K 642
KK587 982
A8A3Q 795
A333A 630
79999 827
587A6 587
5888J 997
TTT77 647
QQQ88 412
6346T 798
JQQT9 507
AA223 872
75475 590
K8374 680
72252 464
Q7775 594
KAKQK 973
5T4T4 353
JQ3JQ 139
99334 1
646K4 90
6AKA5 536
T3KA9 296
63QT9 11
Q8AJ3 308
AAA8A 922
J7722 954
T8A32 891
QAQQA 75
T9TJ9 424
A8573 786
2T33J 485
85T6T 834
255QQ 405
TJT8T 131
89289 408
9626T 515
8J2A2 721
J4522 812
7T757 79
9A999 398
554AJ 217
KKKK8 362
JQTJ3 746
K8J76 960
TK6QA 706
5Q895 976
33993 690
9699A 320
A8265 640
66AAJ 958
66664 48
59Q2K 661
J8668 324
9AJ25 889
862Q8 546
8K9T9 198
K3K3K 215
JTJ77 689
AJTT5 377
88JJ7 448
8737J 688
7KQKQ 51
KKK5K 295
79T9T 757
59A47 502
43AJ9 573
K9792 605
37Q33 824
TTT6T 737
A4A22 486
652KK 478
AK42Q 990
Q44J5 853
5Q999 902
99JJJ 619
JJQKK 714
TA5JK 530
AQQAA 756
A6JAJ 43
55KKK 832
36A2J 374
4333J 816
A2J9A 461
95Q49 292
JK9A2 365
J5256 93
KK6J6 953
8QJ72 750
95959 845
KK4KK 57
J57Q2 686
5QKA9 119
J5J99 342
KKK2K 454
33A3K 177
688J2 409
QJTTK 766
3K978 741
64T47 603
34584 776
89KT2 251
T893T 653
KK957 717
9Q8Q5 959
K5KQ5 174
3Q223 456
2J9Q4 934
A37AQ 58
42Q35 712
33633 809
57J3J 326
58959 867
Q838J 632
QQKJK 936
8Q7T7 238
T2T2T 491
2T987 391
69KQT 6
Q6J98 815
A4JQ4 299
KAKAJ 25
8765J 792
9492J 272
5QQK5 64
J9966 348
8K888 418
K8833 813
33853 129
24344 321
Q4444 970
44QQT 55
82T28 547
97K7Q 262
3A34A 255
K5K9K 567
QT4QT 65
84484 211
K54QT 471
23K3K 951
KTQ2J 157
JQKKK 705
JJ333 673
44222 137
26K72 991
T59KT 34
TQ7QT 684
J9J99 435
26648 950
T4TKA 758
6K66K 294
5JQ55 85
392QT 387
54A7J 773
99Q9T 903
56365 692
44T24 490
538Q4 261
65277 703
44A44 941
J2A34 535
7J733 649
2422T 467
3J9AJ 554
3KT3K 389
A999A 288
2Q5J2 163
98685 599
35555 944
6Q2A6 35
TTTTJ 489
26868 676
QJJ67 833
37383 133
K7KK7 781
9J77K 617
QAAAA 634
QQJQ8 354
JK954 778
JKK5K 957
36263 227
82729 687
9999T 9
6A8A8 679
KJ56A 978
89926 388
KTKTT 223
555J5 522
6TJTT 317
69KJ6 29
QAT85 638
852T3 12
35666 42
822J5 416
7K2J7 432
38Q2K 586
5J5A7 278
39257 107
J4TTJ 857
T88A8 595
68A7J 695
9T7TT 239
Q3J93 920
J3AJA 94
44333 516
2KQK2 791
T7AKA 628
KTK2J 92
4A72T 528
55JQ6 146
AAAJA 338
66JQ2 881
34394 169
82885 194
A383T 503
888TT 228
7796J 560
A7KJ2 4
22253 614
9A868 735
633J6 185
A53Q5 724
QJ993 68
6J446 900
752J5 425
64295 707
QQQQJ 952
94244 210
4AK69 473
579Q4 822
46A75 22
93333 588
J8992 771
Q32KQ 517
3KAQ7 668
23685 804
48J84 346
QTTQT 147
949J9 233
75A34 160
2365T 660
4J444 259
Q748K 469
AA85Q 282
588TT 966
TJ8TJ 519
7747A 279
98TK4 907
49644 540
22953 141
KK5K7 331
J5255 122
QJA8Q 794
47J44 892
85J85 480
867JQ 30
AAAA5 40
T6TKJ 91
4A77Q 871
3333T 916
Q44Q5 66
J6669 722
JK9J9 591
QKKKK 218
37A6A 615
J3227 134
T9T4T 109
J5QQJ 610
43K89 927
QT86T 287
44JTJ 947
7JJ23 301
26966 799
77J67 760
AKAAA 440
9JA4A 156
KK6KT 172
Q8743 894
22252 229
T55JQ 404
76KK3 606
37733 755
989JQ 132
977J9 476
KAKKK 179
4K2T5 170
834J4 178
45QQQ 801
886J7 667
48244 972
6J387 243
7JJ7Q 437
4558Q 268
27333 144
QQ2QQ 774
A33AA 44
44474 989
573K5 897
99339 54
3838Q 696
QJ85K 777
3388J 652
Q3T94 678
743JT 216
A8A82 494
8A98A 250
66996 77
A43A8 350
9844Q 493
QA688 62
6J695 556
5593K 743
5K556 861
JAQQA 414
98T9T 298
34Q3Q 431
TKQAQ 618
55255 110
222T2 852
QQA88 940
27965 322
77K4K 97
57757 361
63A3T 121
2KAKA 985
77778 840
97TTJ 888
Q5558 307
88828 681
984JK 748
66766 273
494TJ 779
TAT37 70
7Q434 481
3JJ3J 725
TTTT3 477
9QT9T 955
65877 805
39965 452
J8J52 698
86866 330
63996 995
55559 571
J42JK 539
QTKJA 315
99579 662
TAT55 731
72JQQ 497
2ATKT 765
22992 108
7A4KJ 430
JJ8JJ 700
Q939A 80
A83A8 971
8JA66 393
7Q77J 742
74247 654
T6287 810
J777J 576
82J28 124
69699 860
5QQ5Q 967
23J3K 862
T9T9T 333
43434 314
QQQ37 15
52235 415
496Q4 674
K4444 23
49942 575
94999 863
AT5K9 138
66A65 645
7242T 558
44545 942
24444 19
88A88 158
K5498 175
JJ8Q8 521
52QAQ 811
8AA3J 284
85588 831
39Q7Q 126
QJ6Q3 913
93599 767
KK8K7 663
8AAA8 525
A7666 669
Q7QQ5 996
28J84 666
J5A7A 335
38TKK 988
94494 352
2266J 26
T8J23 783
K78T6 235
T3533 658
8Q8Q8 974
2QJJQ 719
6T72Q 84
AT8KJ 924
QQ529 190
5A355 682
AQ793 975
T4427 675
QQ25Q 180
AJAAQ 125
8885Q 443
JT6T7 933
Q9732 780
32332 939
TK443 181
QQJQK 274
9472K 849
23222 543
JJ4A6 754
Q5A2K 280
68444 956
969Q2 621
759T5 844
9QT2K 390
23J39 212
QQ2JA 529
528TQ 153
83T63 349
JKQ7J 433
45494 613
29998 113
K94JQ 106
777J7 188
K67JA 864
AK5KK 2
K4889 347
27656 797
T7T76 511
76JA6 167
4544J 566
AT6JK 541
6A952 45
AJ247 866
Q6Q86 580
2K22K 61
7T93K 723
94222 548
K888J 401
8J393 267
AAAA4 740
66QQQ 7
57A9J 512
TTTKQ 969
K7777 441
Q8AAK 946
3J336 802
8T888 581
4JK22 406
QT29J 450
628QT 787
JJK44 626
69999 434
3AA3K 732
JK535 230
88J89 622
29999 992
9QQTQ 328
KK5J5 574
99958 205
8AJ53 88
5A465 411
74J47 337
2222Q 747
6349T 111
JKKKK 789
A7K8J 730
6A27K 403
94Q3Q 173
4J226 289
2364Q 154
7777Q 446
TQA65 325
52353 807
44T4T 938
66838 323
K3346 513
9TJT3 300
2AQ2A 100
3J337 46
4T686 18
28TAK 41
95559 263
5T55A 624
KAKK2 963
J2A2A 655
T9998 336
57A5A 164
98822 199
2JT52 242
4K3T9 878
TQ683 830
44T44 514
555T5 854
74448 399
Q723K 657
5922J 275
A6966 733
KK6KK 253
K93J9 472
TTQQQ 313
Q322Q 413
Q422J 948
7AQ77 16
9229K 281
62Q69 474
A3A86 839
74J7T 585
A9A9A 899
88688 589
348Q8 196
76K24 759
T59TT 422
7Q749 716
66552 112
QJ4QQ 208
29K7T 572
33A8J 204
3Q32J 140
9J999 526
769K3 59
8J77T 533
KKTKK 501
522K5 998
79979 850
KJ982 883
AA8A3 504
J5J55 699
T237A 220
J2KK2 159
96992 935
6655J 713
Q6777 436
43QK5 36
3T3J3 445
23333 500
86593 636
Q9T28 499
AA5QA 283
67766 340
K5555 224
553A4 459
97J44 697
3A579 38
332JA 977
3QK56 69
Q4QJ9 664
99QQQ 488
62266 818
77J97 904
JJ278 592
374A9 117
787J7 659
6QQ66 793
49Q8A 376
59AA5 550
76AT5 616
3A999 727
74T6A 744
65783 510
7222T 600
8AT4Q 356
44434 720
5J3AA 551
Q8A7A 763
66373 708
2Q374 874
59945 458
3JA73 89
252QQ 316
26869 885
9QQ5Q 496
97T63 290
53J57 14
66464 520
AJAAJ 553
44446 561
49J94 930
24K33 50
75AAA 625
66236 492
65J55 370
8Q87T 327
J2K2T 790
3QKQ3 803
9T599 343
23QJ9 397
69562 814
2457T 189
88352 694
KAT73 380
T657T 74
2AJQ4 856
JQ9J9 984
33337 451
8AJ89 302
999QQ 200
66366 770
75QJ5 423
T9TQT 73
T638T 265
93JJ9 987
535Q5 534
9QQJQ 583
45444 709
96556 870
A2J2K 240
58335 584
66665 620
A6K37 444
8888J 341
J3TJ5 182
2QQ28 453
7K58Q 505
T78K4 360
TAAAJ 762
66A66 202
K9K89 27
J7T2Q 293
T7653 704
TTT7T 559
33QQ3 929
TQJ26 462
859A7 715
A8AA4 918
J5AAA 979
42A74 305
JA536 1000
64KKK 384
25889 685
7774Q 882
88Q88 449
7Q8JT 396
T86JA 736
72888 266
99TT9 176
KQ4KQ 438
77766 197
6662Q 602
K888K 842
AT4A7 487
A4J48 128
ATK29 672
T69J7 297
46556 835
77427 319
7A777 98
JJ44J 843
22TT2 72
A9K82 120
9KKKK 923
37272 191
AQK48 369
88933 447
AAT68 785
29222 123
AAKKK 886
3J66A 598
KQKKQ 911
88988 769
8424J 187
2J22J 455
J3753 310
JKJ4K 368
55556 468
44K8A 745
65AA8 13
QQQ7Q 168
QA342 378
KQQKQ 508
K79Q2 460
Q797J 926
66K65 285
A9325 848
96JAJ 601
T77T7 527
444Q5 277
KTAQ4 734
574K7 884
797T7 928
J8JJ8 817
5J454 739
3684Q 95
837QT 332
54459 921
8JJA9 39
A2229 869
7T824 442
QA977 531
54545 236
86882 838
J6466 524
AAJ2A 728
7KJ99 549
T47T4 829
TTQTT 192
T4TTT 648
5QQJ5 896
73TTT 375
82T92 749
2J32J 245
KQKK7 271
3333J 130
2JTJ2 577
5TT55 847
7A92K 466
92K4J 887
TT664 81
JKQQ5 83
449T4 484
TQ3T3 8
23569 643
222J2 392
32QTQ 379
T5T89 99
AQKAK 656
QQ66J 479
24424 901
J552J 729
TT26A 665
28522 965
AA8JJ 419
A7K44 751
6T66Q 545
33663 564
QTK76 114
JTTJT 231
J2J28 402
9ATQA 219
TKTK4 819
333K3 214
TTAT6 910
4J44J 102
34KQA 162
T3338 382
9995J 428
66336 943
62A25 206
49269 937
6T9J5 86
73375 544
3JQJ9 825
6QJ77 993
A2333 537
A393T 246
KKJKJ 623";

    let mut hands = input.lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (split.next().unwrap(), split.next().unwrap())
        }).map(|(hand, bet)| {
            let mut scores: HashMap<char, i32> = HashMap::new();

            hand.chars()
                .filter(|c| *c != 'J')
                .for_each(|c| {
                    match scores.get(&c) {
                        Some(x) => scores.insert(c, x + 1),
                        None => scores.insert(c, 1)
                    };
                });

            let jacks = hand.chars().filter(|c| *c == 'J').count() as i32;

            let max = scores.iter().max_by(|a, b| a.1.cmp(&b.1));

            match max {
                Some((key, value)) => scores.insert(*key, value + jacks),
                None => scores.insert('J', 5)
            };

            let score = scores.into_iter()
                .map(|(_, score)| {
                    match score {
                        2 => 1,
                        3 => 3,
                        4 => 5,
                        5 => 6,
                        _ => 0
                    }
                }).sum::<u8>();

            Hand { score, hand: hand.to_string(), bet: bet.parse::<u32>().unwrap()}
        })
    .collect::<Vec<Hand>>();
    hands.sort();
    let out: u32 = hands.iter()
        .enumerate()
        .map(|(i, hand)| {
            println!("i: {} hand: {} bet: {} score: {}", i, hand.hand, hand.bet, hand.score);
            (i as u32 + 1) * hand.bet
        })
        .sum();

    println!("Hello, world!{}", out);
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self.score.cmp(&other.score) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                match self.hand.chars().zip(other.hand.chars())
                    .map(|(self_char, other_char)| {
                        (char_to_value(self_char), char_to_value(other_char))
                    }).find(|(self_value, other_value)| {
                        self_value != other_value
                    }) {
                        Some((self_value, other_value)) => self_value.cmp(&other_value),
                        None => Ordering::Equal
                    }
            }
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn char_to_value(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0,
        'T' => 10,
        _ => c.to_digit(10).unwrap()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score) && self.hand.eq(&other.hand)
    }
}

impl Eq for Hand {}

struct Hand {
    score: u8,
    hand: String,
    bet: u32
}
