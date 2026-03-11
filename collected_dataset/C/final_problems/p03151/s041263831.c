#include <stdio.h>
#include <stdlib.h>

int asc(const void *a, const void *b) {
  return *(long *)a - *(long *)b;
}

int main(){
   long a[100001],b,d;
   int i,n,dn;
    
   scanf("%d",&n);
   for (i=0;i<n;i++) scanf("%ld",&a[i]);
   d=dn=0;
   for (i=0;i<n;i++) {
      scanf("%ld",&b);
      a[i]-=b;
      if (a[i]<0) {
         d+=a[i];
         dn++;
      }
   }
   if (d==0) {
      puts("0");
      return 0;
   }
   qsort(a,n, sizeof(long),asc);
   for (i=n-1;i>=0 && d<0;i--) {
      if (a[i]<0) {
         puts("-1");
         return 0;
      }
      d+=a[i];
   }
   printf("%d\n",dn+n-i-1);
   return 0;
}
