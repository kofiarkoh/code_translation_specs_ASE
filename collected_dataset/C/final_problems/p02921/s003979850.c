#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

int main(){

  char passado[4],atual[4];
  int cont=0;

  for(int i=0;i<4;i++)
    scanf("%c",&passado[i]);

    for(int i=0;i<4;i++)
      scanf("%c",&atual[i]);

    for(int i=0;i<3;i++){
      
      if(passado[i]==atual[i])
        cont++;
    }

    printf("%d\n",cont);
    return 0;
}
