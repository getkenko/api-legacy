# Najwazniejsze
- Rust + Axum + PostgreSQL (accounts, user data, meal tracking) + MongoDB (product & meal database)
- A LOT of unit & integration tests, so that we for sure know everything works (built-in rust tests + hurl.dev)
- Add unit tests to DTO & database schemas in case we change something and forgot to update it in front/back
- Separate repositories for development and production, development code will have to be tested in order to be merged with prod, once merged with prod, the CI/CD will publish it to the server
- API integration with Postman (i.e. testing, documentation)
- CI/CD with code checks, vulnerability checks, build checks
- Logging + Monitoring + Error Tracking

# Brief
## **1. Konta użytkowników**

* **Zakładanie konta**: email, nazwa użytkownika, hasło
* **Logowanie**: email, hasło
* **Bezpieczeństwo**:

  * Opcjonalne 2FA lub passkeys
* **Odzyskiwanie hasła**
* **Usuwanie konta**:

  * Konto trafia do kolejki usunięcia z 14-dniowym okresem na anulowanie
* **Anulowanie usunięcia konta** (reaktywacja w ciągu 14 dni)

---

## **2. Produkty**

### **Zarządzanie produktami**

* **Wyświetlanie w wyszukiwarce**:

  * kod kreskowy
  * nazwa produktu
  * producent
  * gramatura
  * miara (g, ml, opakowania, porcje, sztuki itd.)
  * makroskładniki
  * skład produktu
  * *opcjonalnie* zdjęcie
* **Dodawanie produktu do dnia/posiłku**: wszystkie powyższe informacje
* **Usuwanie produktu z dnia/sekcji**
* **Ulubione produkty**: możliwość zapisywania
* **Najczęściej wyszukiwane produkty**: zapisywane i sortowane rosnąco (ascending)
* **Recently added**: zapamiętywanie ostatnio dodanych produktów do danej sekcji, wyświetlane przy kliknięciu „+” w danej sekcji

---

### **Dni i sekcje (posiłki)**

* **Format daty**: YYYY-MM-DD

* **Zarządzanie sekcjami**:

  * Dodawanie nowych sekcji (limit: 10)
  * Usuwanie sekcji
  * Przenoszenie sekcji
  * Wyłączanie sekcji bez usuwania

* **Produkty i przepisy w dniach**:

  * Zapisywanie całej sekcji jako **przepis**
  * Dodawanie całej sekcji (jako przepis) do innego dnia/sekcji
  * Dodawanie przepisu jako pojedynczy "produkt"
  * Przy dodawaniu produktów/przepisów: wybór jednostki (1x produkt, gramy, ml, opakowanie, porcja, itd.)
  * Przenoszenie produktów między sekcjami lub dniami
  * Kopiowanie produktów między sekcjami lub dniami
  * Usuwanie wszystkich produktów z sekcji lub z całego dnia
  * Udostępnianie: dnia, produktu, przepisu

---

## **3. Przepisy**

* **Dane przepisu**:

  * Zdjęcie (opcjonalne)
  * Makroskładniki
  * Czas przygotowania
  * Instrukcje przygotowania
  * Składniki (przeliczanie na x porcji)
  * Tagi (np. "wege", "szybkie", "wysokobiałkowe")
  * Autor przepisu
  * Opinie (1 ocena na użytkownika)
  * Reportowanie przepisów

* **Zarządzanie przepisami**:

  * Tworzenie własnych przepisów
  * Przepisy użytkownika
  * Przepisy od aplikacji (zweryfikowane)
  * Wyszukiwanie przepisów i produktów
  * Dodawanie składników przepisu do posiłku (np. "dodaj składniki do śniadania w czwartek, 29 maja")
  * Weryfikacja przepisów

---

## **4. Pomiar ciała (Body Measurements)**

* **Dane**:

  * masa ciała, szyja, klatka, talia, brzuch, biodra, poziom tłuszczu, BMI itd.
* **Okresy do analizy**:

  * od początku używania, ostatni rok, 6 miesięcy, 3 miesiące, 1 miesiąc, wybrany zakres dat
* **Wyświetlanie**:

  * data, pomiar (kg/lb), zmiana (+/-)

---

## **5. Dieta i preferencje**

* **Wybór diety**:

  * wpływa na proponowane przepisy (np. wege, keto, bez laktozy)

---

## **6. Lista zakupów**

* Generowana na podstawie produktów i przepisów
* customowa lista zakupów

---

## **7. Habits Tracking**

* Przykład: szklanka wody po przebudzeniu
* Definiowanie habitów: nazwa, częstotliwość, czas trwania

---

## **8. Podsumowania i analizy**

* **Zakres**: dzień, tydzień, wybrany okres
* **Informacje**:

  * dni z pełnym logowaniem
  * podsumowanie kalorii, białka, tłuszczy, węglowodanów, witamin itd.
  * porównania (np. najlepsze dni, średnie spożycie)

---

## **9. Ustawienia użytkownika**

* **Wygląd aplikacji** (jasny/ciemny motyw)
* **Profil**:

  * nazwa, email, hasło, płeć, data urodzenia, wzrost
* **Cele**:

  * aktualna waga → docelowa waga
  * tryb (redukcja / masa / utrzymanie)
  * tempo zmiany wagi
  * aktywność dzienna
  * aktywność treningowa
  * dzienne cele (kcal, białko, tłuszcz, węglowodany – w gramach i % z łącznego spożycia, domyślnie 25/25/50)
  * indywidualne cele dla każdego dnia tygodnia
* **Plan posiłków**:

  * definiowanie sekcji
  * godziny posiłków + powiadomienia
* **Język aplikacji**: domyślnie polski, z planowaną ekspansją
* **Śledzenie wody**
* **Przypomnienia o posiłkach**
* **Porównanie zdjęć „przed i po”**
* **Śledzenie treningów** (w stylu aplikacji Stronger na iOS)

### **Dni i sekcje (posiłki)**

* **Format daty**: YYYY-MM-DD

* **Zarządzanie sekcjami**:

  * Dodawanie nowych sekcji (limit: 10)
  * Usuwanie sekcji
  * Przenoszenie sekcji
  * Wyłączanie sekcji bez usuwania

* **Produkty i przepisy w dniach**:

  * Zapisywanie całej sekcji jako **przepis**
  * Dodawanie całej sekcji (jako przepis) do innego dnia/sekcji
  * Dodawanie przepisu jako pojedynczy "produkt"
  * Przy dodawaniu produktów/przepisów: wybór jednostki (1x produkt, gramy, ml, opakowanie, porcja, itd.)
  * Przenoszenie produktów między sekcjami lub dniami
  * Kopiowanie produktów między sekcjami lub dniami
  * Usuwanie wszystkich produktów z sekcji lub z całego dnia
  * Udostępnianie: dnia, produktu, przepisu

---

## **3. Przepisy**

* **Dane przepisu**:

  * Zdjęcie (opcjonalne)
  * Makroskładniki
  * Czas przygotowania
  * Instrukcje przygotowania
  * Składniki (przeliczanie na x porcji)
  * Tagi (np. "wege", "szybkie", "wysokobiałkowe")
  * Autor przepisu
  * Opinie (1 ocena na użytkownika)
  * Reportowanie przepisów

* **Zarządzanie przepisami**:

  * Tworzenie własnych przepisów
  * Przepisy użytkownika
  * Przepisy od aplikacji (zweryfikowane)
  * Wyszukiwanie przepisów i produktów
  * Dodawanie składników przepisu do posiłku (np. "dodaj składniki do śniadania w czwartek, 29 maja")
  * Weryfikacja przepisów

---

## **4. Pomiar ciała (Body Measurements)**

* **Dane**:

  * masa ciała, szyja, klatka, talia, brzuch, biodra, poziom tłuszczu, BMI itd.
* **Okresy do analizy**:

  * od początku używania, ostatni rok, 6 miesięcy, 3 miesiące, 1 miesiąc, wybrany zakres dat
* **Wyświetlanie**:

  * data, pomiar (kg/lb), zmiana (+/-)

---

## **5. Dieta i preferencje**

* **Wybór diety**:

  * wpływa na proponowane przepisy (np. wege, keto, bez laktozy)

---

## **6. Lista zakupów**

* Generowana na podstawie produktów i przepisów
* customowa lista zakupów

---

## **7. Habits Tracking**

* Przykład: szklanka wody po przebudzeniu
* Definiowanie habitów: nazwa, częstotliwość, czas trwania

---

## **8. Podsumowania i analizy**

* **Zakres**: dzień, tydzień, wybrany okres
* **Informacje**:

  * dni z pełnym logowaniem
  * podsumowanie kalorii, białka, tłuszczy, węglowodanów, witamin itd.
  * porównania (np. najlepsze dni, średnie spożycie)

---

## **9. Ustawienia użytkownika**

* **Wygląd aplikacji** (jasny/ciemny motyw)
* **Profil**:

  * nazwa, email, hasło, płeć, data urodzenia, wzrost
* **Cele**:

  * aktualna waga → docelowa waga
  * tryb (redukcja / masa / utrzymanie)
  * tempo zmiany wagi
  * aktywność dzienna
  * aktywność treningowa
  * dzienne cele (kcal, białko, tłuszcz, węglowodany – w gramach i % z łącznego spożycia, domyślnie 25/25/50)
  * indywidualne cele dla każdego dnia tygodnia
* **Plan posiłków**:

  * definiowanie sekcji
  * godziny posiłków + powiadomienia
* **Język aplikacji**: domyślnie polski, z planowaną ekspansją
* **Śledzenie wody**
* **Przypomnienia o posiłkach**
* **Porównanie zdjęć „przed i po”**
* **Śledzenie treningów** (w stylu aplikacji Stronger na iOS)

# Przykładowy produkt
Kod kreskowy: 8595588200758 ( typ kodu "ean" )
Nazwa: Pudding proteinowy (Go Active)
Ilość: 200 g
Wielkość porcji: 1 portion (200 g) / w przypadku np sera, szynki itp fajnie jakby dało radę dać 1 slice.
Składniki: __Mleko__ 87 %, białka __mleka__, skrobie modyfikowane, regulator kwasowości: fosforany sodu; aromaty naturalne, barwniki: karmel, karoteny; substancja zagęszczająca: karagen; substancje słodzące: sukraloza, acesulfam K; sól morska. (alergeny podkreślone \_\_blabla\_\_)
Alergeny: Mleko
Nutri-Score
Wartości odżywcze (całe opakowanie i porcja):
- Wartość energetyczna: kj / kcal
- Tłuszcz: g
-- Nasycone: g
- Węglowodany: g
-- w tym cukry: g
- Błonnik: g
- Białko: g
- Sól: g
- Wapń: mg
- Witaminy itd
Vegan/vegetarian: boolean
tags: ["no-gluten", "no-milk", "no-sugar"] 
added by: username
date added
last edited
edited by[?] wszystkie osoby wypisane

# 🧍‍♂️ Dane podstawowe użytkownika
1. **Username** — *string*  
   Przykład: `"john_doe"`

2. **Email** — *string (email)*  
   Przykład: `"ktos@gdzies.pl"`

3. **Password** — *string (hashed)*  
   ...

4. **Display Name** — *string*  
   defaultowo pobierany z username (do zmiany w ustawieniach)

5. **Profile Picture** — *string*  
   Przykład: `"https://example.com/profiles/john.jpg"`

6. **Gender** — *string*  
   Przykład: `"male" | "female"`

7. **Birth Date** — *date (YYYY-MM-DD)*  
   Przykład: `"1990-05-14"`

8. **Age** — *integer*  
   Przykład: `34`

## 📅 Dane konta
9. **Account Creation Date** — *datetime*  
   Przykład: `"2023-06-01T10:15:00Z"`

9.1 **account creation date**
monday, 31 january 2025 godzina

10. **Account Status** — *enum*  
    Przykład: `"active" | "suspended" | "deleted" | "inactive"`

11. **Account Deletion History** — *list of objects*  
    Przykład: `[{"date": "2024-04-01", "reason": "brak motywacji", "type": "built-in albo other jak kliknie other i swoje wpisze"}]`

12. **Last Login Date** — *datetime*  
    Przykład: `"2025-06-02T08:34:00Z"`

13. **Avg. Session Duration** — *duration (minutes)*  
    Przykład: `18`

14. **Timezone** — *string*  
    Przykład: `"Europe/Warsaw"`

15. **2FA (Two-Factor Auth)** — *boolean*  
    Przykład: `true`

16. **IP Address** — *string (IPv4/IPv6)*  
    Przykład: `"192.168.1.1"`

17. **Device Info** — *object*  
    Przykład: `{"name": "iPhone 13", "type": "mobile", "OS": "iOS 17", "screenRes": "1170x2532"}` i wiele wiecej wszystko co sie da i co valuable

18. **Location** — *object*  
    Przykład: `{"city": "Warszawa", "country": "PL", "lat": 52.2297, "lon": 21.0122}`

19. **Location History** — *list of objects*  
    Przykład: `[{"date": "2024-10-10", "location": "Kraków"}]`
    
## 🔒 Prywatność i preferencje
20. **Privacy Settings** — *object*  
    Przykład: `{"showProfile": false, "shareData": true}` i jakies inne to wstepne 

21. **Notifications** — *object*  
    Przykład: `{"email": true, "push": false}`

26. Accessibility od wibracji itd

22. **Data Sharing Preferences** — *object*  
    Przykład: `{"withPartners": false}` i jakie sinne nwm

23. **Language Preferences** — *string*  
    Przykład: `"pl_PL"` 

24. **Theme** — *string*  
    Przykład: `"dark" | "light"`

24.1 nie wiem czy wprowadzac ale fajny bajer moze byc z akcentami albo sie zrobi jak na discordzie custm themes

24.2 app icon

25. **Metric** — *string*  
    Przykład: `"kg" | "lbs"`

## 📊 Aktywność i zachowania
26. **Engagement Metrics** — *enum*  
    Przykład: `"daily" | "weekly" | "monthly"`

27. **Overall Behavioral Data** — *object*  
    Przykład: `{"peakActivityTime": "18:00", "mostUsedDevice": "iPhone", "sessionsPerWeek": 5}`

28. **Response to Notifications** — *object*  
    Przykład: `{"openRate": 0.8}`

29. **Device Switching** — *list of strings*  
    Przykład: `["iPhone", "PC", "iPad"]`

## 💬 Interakcje
30. **Referral Source** — *string*  
    Przykład: `"Facebook Ad"`

31. **User Feedback** — *list of objects*  
    Przykład: `[{"date": "2024-11-12", "feedback": "Great app!"}]`

32. **Customer Support Interactions** — *list of objects*  
    Przykład: `[{"ticketID": "12345", "status": "resolved"}]`

## 🍎 Zdrowie i cele
33. **Fitness Goal** — *string*  
    Przykład: `"Lose weight"`

34. **What Would You Like to Accomplish** — *string*  
    Przykład: `"Zrzucić 10 kg w 3 miesiące"`

35. **What’s Stopping You From Reaching It** — *string*  
    Przykład: `"Brak czasu"` - to z onboarding i 34. tez

36. **Health Data (from connected apps)** — *object*  
    Przykład: `{"steps": 8000, "sleep": "7h"}` i wiecej wrazliwych danych >w<

37. **Intermittent Fasting** — *boolean or object*  
    Przykład: `true | {"start": "18:00", "end": "10:00"}`

38. **Activity Level (w/o workouts)** — *enum*  
    Przykład: `"low" | "moderate" | "high"` i inne opcje sa to potem napisze bo teraz mi sie nie chce

39. **Activity Level (w/ workouts)** — *enum*  
    Przykład: `"moderate" | "high"` i wiecej jest tez 

40. **Weight Change Tempo a week** — *string albo int*  
    Przykład: `"0.5"` 

41. **Specific Diet** — *string*  
    Przykład: `"Keto"`

## 📐 Pomiar i analiza
42. **Height** — *float (cm)*  
    Przykład: `180.5`

43. **Weight** — *float (kg)*  
    Przykład: `82.3`

44. **Calculated Macro** — *object*  
    Przykład: `{"kcal": 2500, "p": 150, "f": 80, "c": 270}`

45. **Measurements** — *object with history*  
    Przykład: `[{"date": "2024-01-01", "weight": 82, "waist": 90, "bf%": 18.5}]`

46. **Difference of (-40kg/+40kg)** — *object with history*  
    Przykład: `[{"date": "2023-01-01", "change": -12.5}]`

## 🍽️ Jedzenie i nawodnienie
47. **Meal Scheme and Times** — *object*  
    Przykład: `{"breakfast": "08:00", "lunch": "13:00", ...}`

48. **Custom Reminders** — *list of datetimes/strings*  
    Przykład: `["Drink water at 11:00"]`

49. **Good Days** — *list of dates*  
    Przykład: `["2025-05-01", "2025-05-03"]`

50. **Shopping List** — *list of strings*  
    Przykład: `["jajka", "ryż", "kurczak"]`

51. **Habits** — *list of strings*  
    Przykład: `["early_sleep", "drink_water"]`

52. **Water (capacity)** — *float (ml or l)*  
    Przykład: `2500`

53. **Saved Recipes** — *list of recipe IDs/objects*  
    Przykład: `["r1", "r2"]`

54. **Created Recipes** — *list*  
    Przykład: `[{"name": "Owsianka", "calories": 350}]`

55. **Edited Recipes** — *list*  
    Przykład: `[...]`

56. **Liked Recipes** — *list*  
    Przykład: `[...]`

57. **Photo (posiłków, postępów)** — *list of strings (URL/path)*  
    Przykład: `["progress_jan.jpg"]`

58. **Preferred Database** — *string*  
    Przykład: `"USDA" | "Polska Baza Produktów"`

## 🏋️ Treningi
59. **Workout per Week** — *integer*  
    Przykład: `4`

60. **Workout Plan** — *object*  
    Przykład: `{"planName": "FBW", "days": 3}`

61. **Workout Stats** — *object*  
    Przykład: `{"volume": 12500, "reps": 120}`

62. **Routine Schedule** — *object*  
    Przykład: `{"Mon": "FBW", "Wed": "Cardio"}`

63. **Saved Routines / Templates** — *list*  
    Przykład: `["FBW v2", "Push/Pull/Legs"]`

64. **Gym Streak** — *integer*  
    Przykład: `12`

65. **Total Workouts / Reps / PBs** — *object*  
    Przykład: `{"workouts": 150, "reps": 12400, "PBs": 45}`

66. **Menu Design** — *object*  
    Przykład: `{"layout": "grid", "theme": "light"}`

67. **Connected Apps** — *list of strings*  
    Przykład: `["Google Fit", "Apple Health"]`

68. **Purchase History** — *list of transactions*  
    Przykład: `[{"id": "tx123", "amount": 39.99}]`

69. **Billing Data** — *object*  
    Przykład: `{"address": "ul. Przykładowa 1, Warszawa"}`