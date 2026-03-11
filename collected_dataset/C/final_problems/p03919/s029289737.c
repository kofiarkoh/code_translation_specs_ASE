#include <stdio.h>
#include <string.h>
int main(){
  char s[30][30][10];
  int i,j;
  int h,w;
  scanf("%d %d",&h,&w);
  for(i=1;i<=h;i++){
    for(j=1;j<=w;j++){
      scanf("%s",s[i][j]);
    }
  }
  for(i=1;i<=h;i++){
    for(j=1;j<=w;j++){
      if(strcmp(s[i][j],"snuke")==0){
        switch(j){
          case 1: printf("A%d\n",i); break;
          case 2: printf("B%d\n",i); break;  
          case 3: printf("C%d\n",i); break;  
          case 4: printf("D%d\n",i); break;
          case 5: printf("E%d\n",i); break;
          case 6: printf("F%d\n",i); break;
          case 7: printf("G%d\n",i); break;    
          case 8: printf("H%d\n",i); break;
          case 9: printf("I%d\n",i); break;
          case 10:  printf("J%d\n",i); break;
          case 11: printf("K%d\n",i); break;
          case 12: printf("L%d\n",i); break;
          case 13: printf("M%d\n",i); break;
          case 14: printf("N%d\n",i); break;
          case 15: printf("O%d\n",i); break;
          case 16: printf("P%d\n",i); break;
          case 17: printf("Q%d\n",i); break;
          case 18: printf("R%d\n",i); break;
          case 19: printf("S%d\n",i); break;
          case 20: printf("T%d\n",i); break;
          case 21: printf("U%d\n",i); break;
          case 22: printf("V%d\n",i); break;
          case 23: printf("W%d\n",i); break;
          case 24: printf("X%d\n",i); break;
          case 25: printf("Y%d\n",i); break;
          case 26: printf("Z%d\n",i);  
        }
       goto end;
      }
    }
  }
  end:
  return 0;
}