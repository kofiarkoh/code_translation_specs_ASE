#include<stdio.h>
#include<stdlib.h>

int main(){
  int length, sum, i, left = 0,
    right = 0, currentSmallest = 1000000;
  scanf(" %d %d", &length, &sum);
  int *array = (int *) malloc(sizeof(int[length]));
  for ( i = 0; i < length; i++ )
    scanf(" %d", array+i);
  int currentSum = array[0];
  // iterate right until the sum is greater, then iterate left
  while ( 1 ){
    if ( currentSmallest == 1 ){ // can't find a smaller piece than 1
      printf("%d\n", 1);
      return 0;
    }
    int subLength = right-left+1;
    if ( subLength >= currentSmallest ) {
      // subtract the left from currentSum then iterate left
      currentSum -= array[left++];
      continue;
    }
    // reached the goal of exceeding sum
    if ( currentSum >= sum ){ 
      currentSmallest = subLength;
      currentSum -= array[left++];
      continue;
    } else if ( right == length-1 ){
      // reached the case in which the sum does not exist in array
      if ( left == 0 ){
	printf("%d\n", 0);
	return 0;
      }
      printf("%d\n", currentSmallest);
      return 0;
    }
    // iterate right then add right to currentSum
    currentSum += array[++right];
  }
  return 0;
}

