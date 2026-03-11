#include <stdio.h>
int main (){
  while (1){
  int w;
  scanf ("%d",&w); if (w==-1)break;
  if (w<=10){
    printf ("%d",4280-1150);
  }
else if (w<=20){
    printf ("%d\n",4280-1150-125*(w-10));
}
else if (w<=30){
    printf ("%d\n",4280-1150-1250-(140*(w-20)));
}
else {
    printf ("%d\n",4280-1150-1250-1400-(160*(w-30)));
}
}
return 0;
}