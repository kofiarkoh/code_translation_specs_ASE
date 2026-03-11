#include <stdio.h>

int main(int argc, const char *argv[])
{
  int i, j, n;
  int cnt[6];
  double height;

  for(i=0; i<6; i++){
    cnt[i] = 0;
  }

  scanf("%d", &n);

  for(i=0; i<n; i++){
    scanf("%lf", &height);
    if(height >= 185.0) cnt[5]++;
    else if(height >= 180.0) cnt[4]++;
    else if(height >= 175.0) cnt[3]++;
    else if(height >= 170.0) cnt[2]++;
    else if(height >= 165.0) cnt[1]++;
    else cnt[0]++;
  } 

  for(i=0; i<6; i++){
    printf("%d:", (i+1));
    for(j=0; j<cnt[i]; j++){
      printf("*");
    }
    printf("\n");
  }


  return 0;
}