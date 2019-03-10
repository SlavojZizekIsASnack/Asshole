#include <vector>

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

extern "C" Card tick(Card *ptr, size_t len)
{
	std::vector<Card> ss;
	ss.assign(ptr, ptr + len);

	return ss[3];
}