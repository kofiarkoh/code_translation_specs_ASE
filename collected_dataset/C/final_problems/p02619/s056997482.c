#include<stdio.h>
#define ck 26

int d;
int c[32];
int s[512][32];
int res[512];

int eval(int brk){
  int i,j,lc[32],r=0;
  for(i=1;i<=ck;i++){lc[i]=-1;}
  for(i=0;i<d;i++){
    r+=s[i][res[i]];
    lc[res[i]]=i;
    for(j=1;j<=ck;j++){
      r-=c[j]*(i-lc[j]);
    }
    if(i==brk){break;}
  }
  return r;
}

int main(){
  int i,j;
  scanf("%d",&d);
  for(i=1;i<=ck;i++){scanf("%d",&c[i]);}
  for(i=0;i<d;i++){
    for(j=1;j<=ck;j++){
      scanf("%d",&s[i][j]);
    }
  }
  for(i=0;i<d;i++){
    scanf("%d",&res[i]);
    printf("%d\n",eval(i));
  }
}