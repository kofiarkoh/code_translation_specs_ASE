#include<stdio.h>
#define N 1000

int main(void){
  int i,j,num,bi,ci;
  int a[N][10],b[N][10]={0},c[N][10]={0};
  char sb;

  scanf("%d",&num);
  for(i=0;i<num;i++){
    for(j=0;j<10;j++){
      scanf("%d",&a[i][j]);
    }
  }
  for(i=0;i<num;i++){
    bi=0;
    ci=0;
    j=0;
    b[i][bi]=a[i][j];
    bi++;
    j++;
    c[i][ci]=a[i][j];
    ci++;
    j++;
    for(;;){
      if(b[i][bi-1]<a[i][j]){
        b[i][bi]=a[i][j];
        bi++;
        j++;
        if(j==10){
          printf("YES\n");
          break;
        }
      }else if(c[i][ci-1]<a[i][j]){
        c[i][ci]=a[i][j];
        ci++;
        j++;
        if(j==10){
          printf("YES\n");
          break;
        }
      }else{
        printf("NO\n");
        break;
      }
    }
  }
  return 0;
}