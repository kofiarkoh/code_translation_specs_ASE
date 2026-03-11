#include<stdio.h>
#include<limits.h>
#include<stdlib.h>

// checkWeight checks if the packages in belt can be loaded onto k trucks without exceeding the maxWeight.
int checkWeight(int maxWeight, int k, int *belt, int n){
  int i, currentWeight = 0, trucksUsed = 1, minExceed = INT_MAX;
  for ( i = 0; i < n; i++ ){
    int nextWeight = currentWeight + belt[i];
    if ( nextWeight > maxWeight ){
      if ( nextWeight < minExceed )
	minExceed = nextWeight;
      trucksUsed++;
      currentWeight = 0;
      i--;
      if ( trucksUsed > k )
	return minExceed;
    } else
      currentWeight = nextWeight;
  }
  return 0;
}

int main(){
  int i, n, k, minPossible = 0;
  scanf(" %d", &n);
  scanf(" %d", &k);
  int *belt = (int *) malloc(sizeof(int[n]));
  for ( i = 0; i < n; i++ )
    scanf(" %d", belt + i);
  for ( i = 0; i < n; i++ )
    if ( belt[i] > minPossible )
     minPossible = belt[i];
  int answer = 0;
  int currentWeight = minPossible;
  while ( !answer ){
    int holder = checkWeight(currentWeight, k, belt, n);
    if ( !holder ){ // currentWeight is a solution;
      answer = currentWeight;
    } else
      currentWeight = holder*2;
  }
  int check = 0;
  while ( !check )
    check = checkWeight(--answer, k, belt, n);
  printf("%d\n", answer+1);
  return 0;
}

