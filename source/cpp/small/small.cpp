#include <vector>
#include <stdio.h>
#include <iostream>

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
	Clear,
	Single,
	Double,
	Triple,
	Quadruple,
};

template <class T>
struct CVec
{
	T *ptr;
	size_t len;

	vector<T> to_vec()
	{
		vector<T> vec;
		vec.assign(ptr, len + ptr);

		return vec;
	}
};

struct Play
{
	unsigned int start;
	unsigned int length;
};

extern "C" Play
tick_unsafe(CVec<Card> c_hand, PlayType play_type, Card last_card)
{
	vector<Card> hand;
	hand = c_hand.to_vec();

	Play play;

	play.start = 5;
	play.length = 1;

	return play;
}