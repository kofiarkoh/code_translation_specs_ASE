#include <stdio.h>

#define NMAX 100000
#define NMAX1 (NMAX+1)
#define INIT 2147483647

int main(){
   int n;
   int array[NMAX1];
   int iq, nq, cmd, x, y;
   int *p, *end, min;
   scanf("%d %d",&n,&nq);
   end=array+n;
   for(p=array;p!=end;p++) *p=INIT;
   for(iq=0;iq<nq;iq++){
      scanf("%d %d %d",&cmd,&x,&y);
      switch(cmd){
      case 0 :
         array[x]=y;
         break;
      case 1 :
         p=array+x;
         end=array+y;
         min=*p;
         for(;p!=end;p++){
            if(*p<min) min=*p;
         }
         printf("%d\n",min);
         break;
      }
   }
   return 0;
}