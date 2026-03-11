#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define df 0
int dc=100000;
#define MAX 100010
#define MOD 998244353
typedef long int li;

char tr[3]={'R','G','B'};
int made[8]={};
// \0,R,G,B,RG,RB,GB,RGB
//  0 1 2 3  4  5  6   7
void find(char c,int* i,int* j){
  if(df)printf("%c ",c);
  int x;
  if(c=='R'){
    if(df)printf("6: %d,2: %d,3: %d",made[6],made[2],made[3]);
    if(made[6]) {*i=6; *j=7; return;}
    else if(made[2]) {*i=2; *j=4; return;}
    else if(made[3]) {*i=3; *j=5; return;}
    else {*i=0; *j=1; return;}
  }
  if(c=='G') {
    if(df)printf("5: %d,1: %d,3: %d",made[5],made[1],made[3]);
    if(made[5]) {*i=5; *j=7; return;}
    else if(made[1]) {*i=1; *j=4; return;}
    else if(made[3]) {*i=3; *j=6; return;}
    else {*i=0; *j=2; return;}
  }
  if(c=='B') {
    if(df)printf("4: %d,1: %d,2: %d",made[4],made[1],made[2]);
    if(made[4]) {*i=4; *j=7; return;}
    else if(made[1]) {*i=1; *j=5; return;}
    else if(made[2]) {*i=2; *j=6; return;}
    else {*i=0; *j=3; return;}
  }
}



int main(){
  if(df)printf("*** debug mode ***\n");
  int n;
  scanf("%d",&n);
  char S[3*n+10];
  scanf(" %s",S);
  if(df)printf("%d %s\n",n,S);
  int i,dec,inc,len=strlen(S);
  li ans=1;
  made[0]=n;
  if(df){
    for(i=0;i<8;i++)
      printf("%d ",made[i]);
    printf("\nlen: %d\n",len);
  }
  for(i=0;i<len;i++){
    find(S[i],&dec,&inc);
    if(df) printf("%d,%d\n",dec,inc);
    ans*=made[dec];
    ans%=MOD;
    made[dec]--;
    made[inc]++;
  }
  
  printf("%ld",ans);
  return 0;
}


/// confirm df==0 ///
