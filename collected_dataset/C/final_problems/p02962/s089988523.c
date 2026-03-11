#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<stdbool.h>
#define base 334334334334334317
#define llinf 4154118101919364364

long long llmax(long long a,long long b){if(a>b){return a;}return b;}

long long ctoll(char x){
  return x-'a';
}

long long dp[524288];
bool isvalid[524288];
bool ispassed[524288];
long long sl,tl;

long long rep(long long pt){
  //printf("<%lld>\n",pt);
  if(ispassed[pt]==true){return llinf;}
  if(dp[pt]!=-1){return dp[pt];}
  if(isvalid[pt]==false){
    dp[pt]=0;
  }
  else{
    ispassed[pt]=true;
    dp[pt]=1+rep((pt+tl)%sl);
    ispassed[pt]=false;
  }
  return dp[pt];
}

int main(){
  long long i,sh,th,k=1;
  long long res=0;
  char s[1048576],t[524288];
  scanf("%s%s",s,t);
  sl=strlen(s);tl=strlen(t);
  for(i=sl;i<1048576;i++){
    s[i]=s[i%sl];
  }
  th=0;
  for(i=0;i<tl;i++){
    th*=26;
    th+=ctoll(t[i]);
    th%=base;
    k*=26;
    k%=base;
  }
  sh=0;
  for(i=0;i<tl;i++){
    sh*=26;
    sh+=ctoll(s[i]);
    sh%=base;
  }
  for(i=0;i<sl;i++){
    if(sh==th){isvalid[i]=true;}
    else{isvalid[i]=false;}
    sh*=26;
    sh%=base;
    sh-=((k*ctoll(s[i]))%base);
    while(sh<0){sh+=base;}
    sh+=ctoll(s[tl+i]);
    sh%=base;
  }
  for(i=0;i<sl;i++){
    ispassed[i]=false;
    dp[i]=-1;
  }
  for(i=0;i<sl;i++){
    res=llmax(res,rep(i));
  }
  if(res>llinf){
    printf("-1\n");
    return 0;
  }
  printf("%lld\n",res);
  return 0;
}
