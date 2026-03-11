#include <stdio.h>
int main(void)
{
    int ha,hb,ka,kb,a,b,c,d,ht,kt;
    scanf("%d %d",&ha,&hb);
    scanf("%d %d",&ka,&kb);
    scanf("%d %d %d %d",&a,&b,&c,&d);
    ht=ha*a+hb*b;
    if(ha<=10)
      ht+=(ha/10)*c;
    if(hb<=20)
      ht+=(hb/20)*d;
    kt=ka*a+kb*b;
    if(ka<=10)
      kt+=(ka/10)*c;
    if(kb<=20)
      kt+=(kb/20)*d;
    if(ht>kt)
    printf("hiroshi\n");
    if(kt>ht)
    printf("kenjiro\n");
    if(kt==ht)
    printf("even\n");
    }




