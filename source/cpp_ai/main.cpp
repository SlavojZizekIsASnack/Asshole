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
	Ten,
	Jack,
	Queen,
	King,
	Ace,
	Two,
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
	vector<Card> ss;
	ss.assign(ptr, ptr + len);

	return ss[3];
}