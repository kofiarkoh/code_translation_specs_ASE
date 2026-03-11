#include<stdio.h>
#include<stdlib.h>

typedef struct a{
  int d,p;
}A;

int main(void)
{
  int i,j;
  int n,m;
  A kukan[10000],tmp;
  int s;

  while(1){
    scanf("%d %d",&n,&m);
    if(n==0 && m==0)break;
    
    for(i=0;i<n;i++)scanf("%d %d",&kukan[i].d,&kukan[i].p);

    for(i=0;i<n-1;i++){
      for(j=i;j<n;j++){
	if(kukan[i].p<kukan[j].p){
	  tmp = kukan[i];
	  kukan[i] = kukan[j];
	  kukan[j] = tmp;
	}
      }
    }

    for(i=0;i<n;i++){
      if(kukan[i].d<m){
	m -= kukan[i].d;
	kukan[i].d = 0;
      }else{
	kukan[i].d -= m;
	m = 0;
      }
      if(!m)break;
    }

    s = 0;
    for(i=0;i<n;i++)s += kukan[i].d * kukan[i].p;
    printf("%d\n",s);

  }
  return 0;
}