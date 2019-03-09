enum Face
{
	Ace,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
};
 
enum Suit
{
	Diamonds,
	Clubs,
	Hearts,
	Spades,
};

struct Card
{
	Face face;
	Suit suit;
};

extern "C" Suit find_suit(Card d)
{
	return d.suit;
}
