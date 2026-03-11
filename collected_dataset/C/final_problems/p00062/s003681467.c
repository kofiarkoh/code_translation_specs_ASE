#include<stdio.h>
#define N 10000

int main(void){
  int i,c,sf,num,j,k,l;
  char n[11];
  int seq[11],ans[N];
  char hoge;

  for(i=0;;i++){
    c=scanf(" %s",n);
    if(c==EOF){
      break;
    }

    for(j=0;j<10;j++){
      seq[j]=n[j]-'0';
    }

    for(j=0;j<9;j++){
      for(k=0;k<9-j;k++){
        seq[k]=(seq[k]+seq[k+1])%10;
      }
    }
    ans[i]=seq[0];
  }
  num=i;
  for(i=0;i<num;i++){
    printf("%d\n",ans[i]);
  }
  return 0;
}