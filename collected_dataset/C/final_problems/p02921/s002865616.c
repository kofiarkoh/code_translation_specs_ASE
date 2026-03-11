#include <stdio.h>
#include <string.h>
int main()
{
    int x=0,i;
    char a[10],b[10];
    scanf("%s", a);
    scanf("%s", b);
    for(i=0;i<3;i++){
        if(a[i] == b [i]){
            x++;
        }
    }
    printf("%d\n", x);
    return 0;
}
