#include<stdio.h>
#include<string.h>
#include<stdlib.h>

int main(void){
  FILE *fp;
  int num,keta[5];
  int i,j;
  while(scanf("%d",&num)!=EOF){
    for(i=0;i<5;i++){
      keta[4-i]=num%10;
      num/=10;
    }
    /*for(i=0;i<5;i++){
      printf("%d",keta[i]);
      }
      putchar('\n');*/
    for(i=0;i<8;i++){
      for(j=0;j<5;j++){
	switch(i){
	case 0:
	  putchar((keta[j]/5)?' ':'*');
	  break;  
	case 1:
	  putchar((keta[j]/5)?'*':' ');
	  break;
	case 2:
	  putchar('=');
	  break;
	case 3:
	  putchar((keta[j]%5)?'*':' ');
	  break;
	case 4:
	  putchar((keta[j]%5-1)?'*':' ');
	  break;
	case 5:
	  putchar((keta[j]%5-2)?'*':' ');
	  break;
	case 6:
	  putchar((keta[j]%5-3)?'*':' ');
	  break;
	case 7:
	  putchar((keta[j]%5-4)?'*':' ');
	  break;
	case 8:
	  putchar((keta[j]%5-5)?'*':' ');
	  break;
	}
      }
      putchar('\n');
    }
    putchar('\n');
  }
  return 0;
}