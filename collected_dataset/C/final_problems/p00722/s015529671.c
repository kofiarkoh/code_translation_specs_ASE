#include <stdio.h>
#include <math.h>

int is_Prime(int n)
{
  if (n < 2)
    return 0;
  if(n == 2)  
    return 1;

    int last = (int) sqrt(n) + 1;  /* conservatively safe */
    int j;
  for(j = 2; j <= last; ++j){
    if (0 == n % j)
      return 0;
  }

  return 1;
}

int main(){
    
    int a,d,n;
    
    while(1){
        scanf("%d %d %d", &a, &d, &n);
        if(a == 0 && d == 0 && n == 0){
           break;
        }
        while(1){
            

            if(is_Prime(a) == 1)
                n--;
                if(n == 0)
                    break;

            a += d;
        }    
        printf("%d\n",a);

    }

}
