# FORR3CG - Skilaverkefni 3 (10%)

- **Verkefnið er einstaklingsverkefni**. Ef tveir eða fleiri nemendur skila sömu lausnunum er gefið 0 (núll) fyrir þær lausnir.
- **Ef kóði er tekinn af netinu** (eða öðrum álíka stöðum) skal taka það fram, benda á hvaðan hann kemur og skrifa skýringar (e. comment) við hverja línu kóðans. Sé það ekki gert verður gefið 0 (núll) fyrir verkefnið í heild.
- **Frjálst val er** um að skrifa skýringar við kóðann, að öðru leiti en þar sem liðurinn hér að ofan kallar á, en þú þarft að geta útskýrt hverja línu kóðans nákvæmlega ef beðið er um. Geti nemandi ekki útskýrt eitthvað í kóðanum sem skilað er er gefið 0 (núll) fyrir verkefnið.
- **Notið** closures og iterators í stað for lykkja.

## Verkefnið

Þú ætlar að skrifa hugbúnað sem líkir eftir aðflugsstjórn. Aðflugsstjórn heldur utan um flugvélar sem eru á leið inn til lendingar á flugvelli, þegar flugvélarnar eru svo komnar ákveðið nálægt flugvellinum tekur flugturninn þar við þeim. 

Hver flugvél er með kallmerki (String), hraða í hnútum (u32) og fjarlægð frá flugvelli í sjómílum (i32). Flugvélin þarf svo að geta reiknað út (fall) hversu margar mínútur eru þar til að hún kemur að flugvellinum.

Síðan eru flugvélarnar geymdar í lista (Vec) sem er raðaður eftir hversu margar mínútur eru þar til þær koma að flugvellinum. Ef flugvél er nær flugvelli en 10 sjómílur er hún tekin úr listanum.

Listinn þarf að eiga föll til að skrá flugvél, prenta allar flugvélar, prenta flugvélar sem eru innan ákveðinnar fjarlægðar og svo fall sem uppfærir staðsetningu flugvéla eftir upphefinn mínútufjölda.

Gerðu svo snyrtilegt notendaviðmót með góðri **villumeðhöndlun**.

Dæmi um notkun:
```bash 
skrá tf-abc 100 300 # skráir TF-ABC, hraði 100 hnútar og fjarlægð 300 sjómílur
skrá tf-ice 200 500
skrá aal5 100 50
skrá tf-efg 100 200
prenta allt
    AAL5, hradi: 100 hn, fjarl.: 50 sm, tími: 30 mín.
    TF-EFG, hradi: 100 hn, fjarl.: 200 sm, tími: 120 mín.
    TF-ICE, hradi: 200 hn, fjarl.: 500 sm, tími: 150 mín.
    TF-ABC, hradi: 100 hn, fjarl.: 300 sm, tími: 180 mín.
prenta 250 # prentar aðeins flugvélar sem eru innan 250 sm. frá flugvelli
    AAL5, hradi: 100 hn, fjarl.: 50 sm, tími: 30 mín.
    TF-EFG, hradi: 100 hn, fjarl.: 200 sm, tími: 120 mín.
uppfæra 40 # lætur 40 mínútur líða
prenta allt # AAL5 farin úr listanum
    TF-EFG, hradi: 100 hn, fjarl.: 134 sm, tími: 80 mín.
    TF-ICE, hradi: 200 hn, fjarl.: 367 sm, tími: 110 mín.
    TF-ABC, hradi: 100 hn, fjarl.: 234 sm, tími: 140 mín.
```

Fyrir flugvél útfæra `Display`, `PartialEq`, `PartialOrd`, `Ord` og `TryFrom`.

Fyrir listann útfæra `Display`.

Skila í einni skrá, `main.rs`, á Innu.
