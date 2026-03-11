#include<stdio.h>

int main(){
  int i,j,n,num,prev;
  char segment[]="0000000";
  char nums[][8]={"0111111","0000110","1011011","1001111",
		  "1100110","1101101","1111101","0100111",
		  "1111111","1101111","0000000"};
  do{
    scanf("%d",&n);
    if(n==-1) break;
    for(prev=10,i=0;i<n;i++,prev=num){
      scanf("%d",&num);
      for(j=0;j<7;j++){
	if(nums[prev][j]==nums[num][j]){
	  segment[j]='0';
	}else{
	  segment[j]='1';
	}
      }
      printf("%s\n",segment);
    }
  }while(1);
  return 0;
}