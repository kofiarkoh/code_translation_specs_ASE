#include <stdio.h>

int main(){
   int n,m,q,a[50],b[50],c[50],d[50],i,j,k;
   int t[10],tt,max;

   scanf("%d %d %d",&n,&m,&q);
   for (i=0;i<q;i++) {
      scanf("%d %d %d %d",&a[i],&b[i],&c[i],&d[i]);
      a[i]--;b[i]--;
   }
   for (i=0;i<n;i++) t[i]=1;
   max=0;
   for (;;) {
      tt=0;
      for (k=0;k<q;k++)
         if (t[b[k]]-t[a[k]]==c[k]) tt+=d[k];
      if (tt>max) max=tt;
      for (j=1;j<n;j++)
         if (t[j-1]<t[j]) {
            t[j-1]++;
            for (k=0;k<j-1;k++) t[k]=1;
            break;
         }
      if (j==n) {
         if (t[n-1]==m) break;
         t[n-1]++;
         for (k=0;k<n-1;k++) t[k]=1;
      }
   }
   printf("%d\n",max);
   return 0;
}
