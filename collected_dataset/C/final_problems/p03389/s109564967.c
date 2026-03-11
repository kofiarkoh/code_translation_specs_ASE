#include <stdio.h>
#include <stdlib.h>

/* ソート関数 */
int int_sort( const void * a , const void * b ) {
  /* 引数はvoid*型と規定されているのでint型にcastする */
  if( *( int * )a < *( int * )b ) {
    return -1;
  }
  else
  if( *( int * )a == *( int * )b ) {
    return 0;
  }
  return 1;
}




int main(void){
    int a[3];
    scanf("%d %d %d",&a[0],&a[1],&a[2]);
      qsort(( void * )a , 3 , sizeof( a[0] ) , int_sort );
    int ans=0;
    while(a[2]-a[0]>1){a[0]+=2;ans++;}
    while(a[2]-a[1]>1){a[1]+=2;ans++;}
    if(a[0]==a[1]&&a[1]!=a[2])ans++;
    else if(a[0]!=a[1])ans+=2;
    printf("%d",ans);
    return 0;
}
