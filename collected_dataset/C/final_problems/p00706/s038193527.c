#include <stdio.h>

int main(void)
{
  int n,w,h,x,y,s,t,i,j,k,l,sum,max;

  for(;;){
    scanf("%d", &n);
    if(n==0) break;

    int per[100][100]={0};
    sum=0;
    max=0;
    scanf("%d %d", &w, &h);
    for(i=0;i<n;i++){
      scanf("%d %d", &x, &y);
      per[x-1][y-1]=1;
    }
    scanf("%d %d", &s, &t);
    for(i=0;i+t-1<h;i++){
      for(j=0;j+s-1<w;j++){
	for(k=i;k<t+i;k++){
	  for(l=j;l<s+j;l++){
	    if(per[l][k]==1) sum++;
	  }
	}
	if(sum>max) max=sum;
	sum=0;
      }
    }
    printf("%d\n", max);
  }

  return 0;
}
