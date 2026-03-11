#include<stdio.h>

int main(void){
  int i,j;
  int n,renum,a[20],c[20];
  int p,g,diff,mindiff;

  for(;;){
    scanf("%d",&n);
    if(n==0)
      break;

    for(i=0;i<n;i++)
      scanf("%d",&a[i]);

    renum=1;
    mindiff=0;
    for(i=0;i<n;i++){
      renum*=2;
      c[i]=0;
      mindiff+=a[i];
    }

    for(i=0;i<renum;i++){
      p=0;
      g=0;
      for(j=0;j<n;j++){
        if(c[j]==0)
          p+=a[j];
        else
          g+=a[j];
      }

      diff=(p>=g)?p-g:g-p;
      if(mindiff>diff)
        mindiff=diff;

      c[0]++;
      for(j=0;j<n-1;j++)
        if(c[j]>=2){
          c[j+1]+=c[j]/2;
          c[j]%=2;
        }
    }

    printf("%d\n",mindiff);
  }

  return 0;
}