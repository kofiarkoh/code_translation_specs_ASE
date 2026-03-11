#include<stdio.h>
#include<stdlib.h>
#include<limits.h>
#include<string.h>

int sequenceLength;
int *sequence;

// find returns the minimum element from left to right of sequence.
int find(int left, int right)
{
  int currentMin = INT_MAX;
  for ( ; left <= right; left++ )
    if ( sequence[left] < currentMin )
      currentMin = sequence[left];
  return currentMin;
}

// update changes the elements from left to right of sequence to the value.
void update(int left, int right, int value)
{
  for ( ; left <= right; left++ )
    sequence[left] = value;
  return;
}

int main()
{
  int length, queries, i;
  scanf(" %d %d", &length, &queries);
  sequenceLength = length;
  sequence = (int *) malloc(sizeof(int[sequenceLength]));
  // set all of sequence to the max value
  for ( i = 0; i < sequenceLength; i++ )
    sequence[i] = (1<<31)-1;
  for ( i = 0; i < queries; i++ )
    {
      int left, right, type;
      scanf(" %d %d %d", &type, &left, &right);
      // if the query is an update
      if ( !type )
	{
	  int value;
	  scanf(" %d", &value);
	  update(left, right, value);
	}
      // if the query is a find
      else
	printf("%d\n", find(left, right));
    }
  return 0;
}

