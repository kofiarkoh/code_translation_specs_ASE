#include<stdio.h>

int main()
{
    int h,w;
    scanf("%d %d", &h, &w);


    while(h!=0 || w!=0) {
      for(int i=1; i<=h; ++i) {
        for(int j=1; j<=(w/2); ++j) {
          if(i%2==0)  printf(".#");
          else        printf("#.");
        }
        if(w%2==1) {
          if(i%2==0) printf(".");
          else       printf("#");
        }

        printf("\n");
      }

      printf("\n");
      scanf("%d %d", &h, &w);
    }

    return 0;
}

