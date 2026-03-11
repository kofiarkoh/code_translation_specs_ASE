#include <stdio.h>
 
int main(){
   int n,a,b,c,k,l[8],aa,bb,cc;
   long i,j,m,p,mp;
    
   scanf("%d %d %d %d",&n,&a,&b,&c);
   m=1;
   for (i=0;i<n;i++) {
      scanf("%d",&l[i]);
      m*=4;
   }
   mp=-1;
   for (i=0;i<m;i++) {
      p=aa=bb=cc=0;
      for (j=i,k=0;k<n;k++) {
         if (j%4==0) {
            if (aa>0) p+=10;
            aa+=l[k];
         } else if (j%4==1) {
            if (bb>0) p+=10;
            bb+=l[k];
         } else if (j%4==2) {
            if (cc>0) p+=10;
            cc+=l[k];
         }
         j=j/4;
      }
      if (aa==0 || bb==0 || cc==0) continue;
      if (aa>a) p+=aa-a;
      else      p+=a-aa;
      if (bb>b) p+=bb-b;
      else      p+=b-bb;
      if (cc>c) p+=cc-c;
      else      p+=c-cc;
      if (mp<0) mp=p;
      else if (p<mp) mp=p;
   }
   printf("%ld\n",mp);
   return 0;
}

