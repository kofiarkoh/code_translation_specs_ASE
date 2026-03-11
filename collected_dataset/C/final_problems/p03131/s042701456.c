#include<stdio.h>

int main(){
  long biscuits=1,yen=0;
  long K,A,B;
  scanf("%ld%ld%ld",&K,&A,&B);

  if (A>=B-1)
  {
    biscuits=K+1;
  }else{
    while (0<K)
    {
      if (K<2){ biscuits++; break;}

      if (biscuits>=A)
      {
        yen=biscuits/A;
        biscuits-=yen*A;
        K-=yen;
      }else{
        K-=A-biscuits;
        biscuits+=A-biscuits;
      }

      if (yen<=K)
      {
        biscuits+=yen*B;
        K-=yen;
      }else{
        while (yen>0)
        {
          if (K>0)
          {
            biscuits+=K*B;
            yen-=K;
            K=0;
          }else{
            yen--;
            biscuits+=A;
            K++;
          }
        }
      }
    }
  }

  printf("%ld",biscuits);

  return 0;
}