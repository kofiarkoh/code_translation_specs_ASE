#include <stdio.h>
 
int main(void){
  	int a[3];
  int i,j;
    for(i=0;i<3;i++){
        scanf("%d", &a[i]);
    }
    int max;
  int min;
  	max = a[0];
    min = a[0];
  for(i=0;i<3;i++){
  	if(max <= a[i]){
      max = a[i];
    }
    if(min >= a[i]){
      min = a[i];
    }
  }
	
  
  if(min == a[1] && max == a[0]){
    printf("Yes");
	return 0;
  }
	if(max == a[1] && min == a[0]){
		printf("Yes");
	return 0;
  }
  
printf("No");
 
    return 0;
}