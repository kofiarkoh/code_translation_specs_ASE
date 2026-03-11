#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define df 0
int dc=100000;
#define MAX 100010
int count[9]={};

int main(){
  if(df)printf("*** debug mode ***\n");
  int n,i;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    int a;
    scanf("%d",&a);
    a/=400;
    if(a<=7)count[a]|=1;
    else count[8]++;
  }
  int s=0;
  for(i=0;i<8;i++){
    s+=count[i];
  }
  

  printf("%d %d",s+(count[8]&&(s==0)?1:0),s+count[8]);
  return 0;
}


/// confirm df==0 ///
