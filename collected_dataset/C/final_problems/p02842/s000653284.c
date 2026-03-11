#include<stdio.h>
int main()
{
   
     long num;
    scanf("%ld", &num);
	long ans = 0;
   int i=0;
  for(i=0;i<num;i++){
	ans = i*1.08;
  	if(ans == num){
		printf("%d",i);
      return 0;
    }
  }
	printf(":(");
    return 0;
}