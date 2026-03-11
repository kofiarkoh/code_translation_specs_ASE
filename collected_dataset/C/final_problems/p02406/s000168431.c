#include<stdio.h>
int main(void)
{
  int n,i,x;
  scanf("%d",&n);

  for(i=1; ; )
    {
      /*xにiを代入*/
      x=i;
      /*xが3の倍数か*/
      if(x%3==0){printf(" %d",x);}
      /*まずxの1の位が3か。そうでないなら10より小さいかを確認。小さいならbreak。大きいなら10の位が3
の倍数かを確かめる*/
      else
	{for( ; ; )
	    {
	if(x%10==3){printf(" %d",x);
	  break;}
	else if(0<=x<=9){break;}
	else{ x/=10;}
	    }
	}
      if(++i>n){break;}
    }
  printf("\n");
  return 0;
}