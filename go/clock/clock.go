package clock

import "fmt"

const day = 24 * 60 // 1440 minutes dans une journée

// Clock représente une heure de la journée stockée en nombre total de minutes
// depuis 00:00, toujours normalisé dans l'intervalle [0, 1440).
type Clock int

// New construit une Clock à partir d'heures et de minutes éventuellement hors
// plage (négatives ou supérieures à un jour). La normalisation utilise l'astuce
// du double modulo : ((x % day) + day) % day.
//
// Pourquoi pas un simple x % day ?
// En Go, l'opérateur % conserve le signe de l'opérande de gauche :
//
//	   90 % 1440 ==   90   -> ok
//	 1500 % 1440 ==   60   -> ok
//	  -10 % 1440 ==  -10   -> KO, on voudrait 1430
//	-1450 % 1440 ==  -10   -> KO, on voudrait 1430
//
// En ajoutant day avant le second modulo, on garantit un résultat positif :
//
//	((  -10 % 1440) + 1440) % 1440 == ( -10 + 1440) % 1440 == 1430
//	((-1450 % 1440) + 1440) % 1440 == ( -10 + 1440) % 1440 == 1430
//	(( 1500 % 1440) + 1440) % 1440 == (  60 + 1440) % 1440 ==   60
func New(h, m int) Clock {
	return Clock(((h*60+m)%day + day) % day)
}

// Add retourne une nouvelle Clock avancée de m minutes. La valeur de m peut
// être négative ou dépasser une journée : la normalisation cyclique est gérée
// par New.
func (c Clock) Add(m int) Clock { return New(0, int(c)+m) }

// Subtract retourne une nouvelle Clock reculée de m minutes. Équivaut à
// Add(-m) et bénéficie de la même normalisation cyclique.
func (c Clock) Subtract(m int) Clock { return New(0, int(c)-m) }

// String formate l'horloge au format 24h "HH:MM" (toujours deux chiffres),
// par exemple "08:05" ou "23:30".
func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", int(c)/60, int(c)%60)
}
