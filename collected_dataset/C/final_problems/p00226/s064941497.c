#include<stdio.h>
#define N 100000

int main(void){
  int i,num,j,k;
  char r[5],a[5];
  char hoge;
  int hit[N],blow[N];

  for(i=0;;i++){
    scanf("%s",&r);
    scanf("%s",&a);
    if(r[0]=='0' && a[0]=='0')
      break;
    hit[i]=0;
    blow[i]=0;

    for(j=0;j<4;j++)
      if(r[j]-'0'==a[j]-'0')
        hit[i]++;

    for(j=0;j<4;j++)
      for(k=0;k<4;k++)
        if(j!=k && r[j]==a[k])
          blow[i]++;
  }
  num=i;
  for(i=0;i<num;i++){
    printf("%d ",hit[i]);
    printf("%d\n",blow[i]);
  }
  return 0;
}