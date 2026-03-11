#include <stdio.h>

int main()
{
    int in, out;
    int n, i, j, k = 0;


scanf("%d", &n);
    
int x[n];

for(i = 0 ; i < n ; i++){
scanf("%d", &x[i]);
}

for(i = 2, j = 3 ; i < n ; i+=2, j+=2){
    if(x[i] != x[i-2]) k++;
    if(x[j] != x[j-2]) k++;
}
if(x[0] == x[1])k = 2;
printf("%d\n", k);
return 0;

}