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

extern "C" Card tick_unsafe(Card *hand_ptr, size_t hand_len, PlayType play_type, Card last_card)
{
	vector<Card> hand;
	hand.assign(hand_ptr, hand_ptr + hand_len);

	return hand[0];
}