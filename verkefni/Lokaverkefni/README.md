# FORR3CG Haust 2024 - Lokaverkefni (20%)

Áður en lengra er haldið skalt þú stofna nýtt **private** repo á github og bjóða *gestskoli* inn sem samstarfsaðila. Farðu svo inn á Innu og skila slóðinni að repo-inu í **Lokaverkefni** skilahólfið.

  - [Almennar reglur varðandi verkefnið](#almennar-reglur-varðandi-verkefnið)
    - [Skýrsla](#skýrsla)
    - [Námsmat](#námsmat)
  - [Skil verkefnisins](#skil-verkefnisins)
  - [Verkefnið](#verkefnið)
    - [Búnaðarlisti Tækniskólans](#búnaðarlisti-tækniskólans)
    - [Grunnkröfur](#grunnkröfur)
    - [Dæmi um aukakröfur](#dæmi-um-aukakröfur)

## Almennar reglur varðandi verkefnið

- Verkefnið er einstaklingsverkefni. Ef tveir eða fleiri nemendur skila sömu lausnunum er gefið 0 (núll) fyrir þær lausnir.
- Ef kóði er tekinn af netinu (eða öðrum álíka stöðum) skal taka það fram, benda á hvaðan hann kemur og skrifa skýringar (e. comment) við hverja línu kóðans. Sé það ekki gert verður gefið 0 (núll) fyrir verkefnið.
  
### Skýrsla

Halda skal utan um verkefnið á github. Þar á að vera allur kóði ásamt dagbók og skýrslu. Í dagbókinni á að halda utan um hvað er gert og hvenær (ef eitthvað er ekki í dagbókinni þá er það ekki í verkefninu). Skýrslan er svo stutt samantekt á dagbókinni ásamt útskýringum á hvernig forritið ykkar vinnur. Þar eiga líka að vera leiðbeiningar fyrir viðmótið sem þið skrifið. Skilið einnig stuttu myndbandi (screen record) þar sem virkni forritsins er sýnd.

:warning: Ekki er gefin sérstök einkunn fyrir skýrsluna og það sem á að vera í henni en vanti eitthvað af því sem á að vera í skýrslunni eða það ekki vel unnið kemur það til mikillar lækkunar á einkunn verkefnisins.

Sniðmát fyrir skýrsluna má finna [hér](./snidmat.md).

### Námsmat

Við mat á verkefninu er horft til eftirfarandi (ekki tæmandi listi):
- Virkni
- Lýsandi nöfn á:
  - Breytum.
  - Föllum og færibreytum þeirra.
  - Struct og enums.
  - Hefðbundnum [ritunarreglum Rust](https://rust-lang.github.io/api-guidelines/naming.html) fylgt.
- Snyrtilega upp settur kóði.
- Skýringar (e. comment) notaðar þar sem það á við.
- Verkefnið unnið jafnt og þétt (commit á github).

## Skil verkefnisins

Verkefnið á að skilast í síðasta lagi á miðnætti laugardaginn 12. október. Ekki verður hægt að fá lengri frest.

## Verkefnið

### Búnaðarlisti Tækniskólans

Húsumsjón Tækniskólans hefur beðið þig um skrifa hugbúnað til að halda utan um ýmsan búnað sem skólinn á og hvar sá búnaður er staðsettur. 

Allur búnaður hefur auðkenni, verðmæti og staðsetningu:
- Auðkennið er einkvæm heiltala (sjálfhækkandi (e. auto increment)).
- Verðmætið er í krónum.
- Staðsetningin sem samanstendur af húsi, hæð og herbergisnúmeri t.d. H-202 sem er þá Háteigsvegur, 2. hæð, herbergi nr. 2 og HA-123 er þá Hafnarfjörður, 1. hæð, herbergi nr. 23. Húsmerkingarnar eru eftirfarandi:
    - HA - Hafnarfjörður,
    - H - Háteigsvegur,
    - S - Skólavörðuholt,
    - X - Annað,

Sá búnaður sem Húsumsjónin vill geta skráð í þessum hugbúnaði eru:
  - Borð, aukalega þarf að skrá hversu margir geta setið við borðið.
  - Stóll, aukalega þarf að skrá hvernig stóllinn er (Hægindastóll, Skólastóll, Skrifstofustóll eða annað).
  - Skjávarpi, aukalega þarf að skrá hversu mörg lumens skjávarpinn er.

### Grunnkröfur

Verkefnið er 100 stig. Fullkomin lausn á grunnkröfum gefur 65 stig eftir því hvaða leið er valin fyrir listann.

Búnaður:

- Gerðu struct fyrir hverja gerð búnaðar og enum þar sem það á við.
- Útfærðu Display og TryFrom fyrir allan búnað.
- Annað sem eðlilegt getur verið að hafa.

Listin:

- Úfærðu struct fyrir listann en listinn heldur líka utan um hvaða auðkenni (id) hver hlutur fær.
  - Það þarf að vera hægt að skrá búnað í listann.
  - Það þarf að vera hægt að eyða búnaði úr listanum.
  - Það þarf að vera hægt að uppfæra staðsetningu búnaðar.
  - Það þarf að vera hægt að prenta út ákveðinn búnað útfrá id.
  - það þarf að vera hægt að prenta á skjá lista með öllum búnaði.
  - það þarf að vera hægt að prenta á skjá lista með öllum búnaði í ákveðnu húsi.
- Gerðu notendaviðmót þar sem hægt er að gera ofantalið, notendaviðmótið á að vera með sama sniði og sýnt var [hér](https://github.com/FORR3CG/H24/blob/main/tímar/bilasala/src/main.rs):
  
### Dæmi um aukakröfur

- Brjóta kóðann upp í margar skrár. (5 stig)
- Prenta á skjá búnað í ákveðinni stofu og ákveðinni hæð í ákveðnu húsi. (5 stig)
- Prenta á skjá búnað af ákveðinni gerð, t.d. alla stóla. (5 stig)
- Gögnin eru röðuð í listanum eftir húsi, hæð, herbergi og svo tegund búnaðar. (5 stig)
- Skrifa og lesa gogn í/úr JSON skrá. (5 stig)
- Annað sem ykkur dettur í hug í samráði við kennara (allt að 10 stig).

