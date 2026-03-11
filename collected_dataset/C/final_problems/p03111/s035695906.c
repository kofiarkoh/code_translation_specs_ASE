#include <stdio.h>
#include <stdlib.h>//random,NULL
#include <string.h>
#include <time.h>
#include <unistd.h>//sleep()
#include <math.h>

#define true 1
#define false 0

#define rep(i,n) for(int i=0; i<((int)n); i++)
#define REP(i,n) for(int i=0; i<=((int)n); i++)
#define srep(i,a,n) for(int i=a; i<((int)n); i++)
#define SREP(i,a,n) for(int i=a; i<=((int)n); i++)
#define rrep(i,n) for(int i=n-1; i>=0; i--)
#define RREP(i,n) for(int i=n; i>=0; i--)


int calculate(int *l_ABC, int N, int A, int B, int C, int *l);
int min(int min_sum, int sum);

int min(int min_sum, int sum){
  if(min_sum>=sum){
    return sum;
  }
  else{
    return min_sum;
  }
}

int calculate(int *l_ABC, int N, int A, int B, int C, int *l){
  int lengtha=0, lengthb=0, lengthc=0;
  int mp=0;
  //最低一つはないといけない。その検査用
  int flaga=0, flagb=0, flagc=0;

  rep(i,N){
    if(l_ABC[i]==0){
      lengtha=lengtha+l[i];
      mp=mp+10;
      flaga=1;
    }
    else if(l_ABC[i]==1){
      lengthb=lengthb+l[i];
      mp=mp+10;
      flagb=1;
    }
    else if(l_ABC[i]==2){
      lengthc=lengthc+l[i];
      mp=mp+10;
      flagc=1;
    }
    else{
      ;
    }
  }

  if(flaga!=1 || flagb!=1 || flagc!=1){
    return 100000;
  }

  mp=mp-30;//多く換算した分を引く

  mp=mp+abs(lengtha-A)+abs(lengthb-B)+abs(lengthc-C);

  return mp;
}


int main(){
  char buf[1024];
  int N, A, B, C;

  fgets(buf,sizeof(buf),stdin);
  sscanf(buf, "%d %d %d %d", &N, &A, &B, &C);

  //l[]長さ
  int *l=(int *)malloc(sizeof(int)*N);
  rep(i,N){
    fgets(buf, sizeof(buf), stdin);
    sscanf(buf, "%d", &l[i]);
  }

  //0:A, 1:B, 2:C, 3:なし
  int *l_ABC=(int *)malloc(sizeof(int)*N);

  //求める最小値
  int min_sum=1000000;
  //途中経過
  int sum;

  rep(i,pow(4,N)){

    int quo=i;
    rep(j,N){
      l_ABC[j]=quo%4;
      quo=(quo-l_ABC[j])/4;
    }

    sum=calculate(l_ABC, N,A, B, C, l);

    min_sum=min(min_sum, sum);
  }

  printf("%d\n", min_sum);

  return 0;
}
