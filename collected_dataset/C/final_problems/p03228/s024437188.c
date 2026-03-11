#include <stdio.h>
int main (void)

{
  int A,B,K,Areturn,l;
  scanf ("%d %d %d",&A,&B,&K);


  if (K%2==1)
  {
  Areturn=(K-1)/2;
  for (l=1;l<=Areturn;l++)
  {
  /*高橋*/
    if (A%2==1)
    {
      A--;
      A=A/2;
      B=B+A;
    }
    else
    {
      A=A/2;
      B=B+A;
    }
  /*青木*/
    if (B%2==1)
    {
      B--;
      B=B/2;
      A=A+B;
    }
    else
    {
      B=B/2;
      A=A+B;
    }
  }
  if (A%2==1)
  {
    A--;
    A=A/2;
    B=B+A;
  }
  else
  {
    A=A/2;
    B=B+A;
  }
  }
  else
  {
  Areturn=K/2;
  for (l=1;l<=Areturn;l++)
  {
    /*高橋*/
      if (A%2==1)
      {
        A--;
        A=A/2;
        B=B+A;
      }
      else
      {
        A=A/2;
        B=B+A;
      }
    /*青木*/
      if (B%2==1)
      {
        B--;
        B=B/2;
        A=A+B;
      }
      else
      {
        B=B/2;
        A=A+B;
      }
    }
  }
  printf("%d %d\n",A,B);

  return 0;
}
