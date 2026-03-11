#include <stdio.h>

int main()
{
    int k,n;
    scanf("%d %d", &k,&n);
    int p[n];
    for(int i=1;i<=n;i++)
    {
        scanf("%d", &p[i]);
    }
     int max = p[1];

  for (int i = 1; i <=n; i++)
  {
    if (p[i] > max)
    {
       max  = p[i];
    }
  }
  int min2;
  int min =min2= 10000;
   
    for (int i = 1; i <=n; i++)
    {
        if(p[i] < min)
        {
           min= p[i];
        }
        else if (p[i] < min2 && p[i] != min) 
            min2 = p[i];
    }
    if(min==0){
        if(max-min2<k-max+min2){
            printf("%d", max-min2);
        }
        else
        printf("%d", k-max+min2);
    }
    else 
        if((max-min)<=(min+k-max))
            printf("%d", max-min);
        else
            printf("%d", k-max+min);
    return 0;
}
