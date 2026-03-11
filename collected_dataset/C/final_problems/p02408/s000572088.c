#include<stdio.h>

int main(){
int n[4][13] ={0};
int j,i,cards,number,k;
char mark;

scanf("%d",&cards);
for(j = 0 ; j < cards; j++){

scanf(" %c %d",&mark,&number);
  if(mark == 'S'){
  n[0][number-1] = 1;
  }else if(mark == 'H'){
  n[1][number-1] = 1;
  }else if(mark == 'C'){
  n[2][number-1] = 1;
  }else if(mark == 'D'){
  n[3][number-1] = 1;
  }
}

for(i = 0; i < 4; i++){
  for(k = 0; k < 13; k++){
    if(n[i][k] == 0){
    if(i == 0){
    printf("S %d\n",k+1);
    }else if(i == 1){
    printf("H %d\n",k+1);
    }else if(i == 2){
    printf("C %d\n",k+1);
    }else if(i == 3){
    printf("D %d\n",k+1);
    }
    }
  }
}


return 0;
}
