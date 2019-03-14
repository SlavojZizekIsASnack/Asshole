#include <vector>

using namespace std;

enum Face
{
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Jack,
	Queen,
	King,
	Ace,
	Two,
	Ten
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

enum PlayType
{
	Single,
	Double,
	Triple,
	Clear,
};

extern "C" Card tick(Card *deck_ptr, size_t deck_len, PlayType play_type, Card last_card)
{
	vector<Card> deck;
	deck.assign(deck_ptr, deck_ptr + deck_len);

	vector<Card> pile;
	deck.assign(pile_ptr, pile_ptr + pile_len);

	return deck[0];
}


