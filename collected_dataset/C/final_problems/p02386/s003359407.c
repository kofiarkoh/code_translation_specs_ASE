#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef struct dice{
    int face[6];
}dice;

void rotate(dice *d, char dir)
{
    int temp;
    switch(dir) {
    case 'N':
        temp=d->face[0];
        d->face[0]=d->face[1];
        d->face[1]=d->face[5];
        d->face[5]=d->face[4];
        d->face[4]=temp;
        break;
    case 'E':
        temp=d->face[0];
        d->face[0]=d->face[3];
        d->face[3]=d->face[5];
        d->face[5]=d->face[2];
        d->face[2]=temp;
        break;
    case 'W':
        temp=d->face[0];
        d->face[0]=d->face[2];
        d->face[2]=d->face[5];
        d->face[5]=d->face[3];
        d->face[3]=temp;
        break;
    case 'S':
        temp=d->face[0];
        d->face[0]=d->face[4];
        d->face[4]=d->face[5];
        d->face[5]=d->face[1];
        d->face[1]=temp;
        break;
    }
}

int isSame(dice d1, dice d2)
{
    static const char dir[]="NNNESSSWNNNESSSWNNNESSSW";
    int i, j;
    for(i=0; i<24; i++) {
        for(j=0; j<6 && d1.face[j]==d2.face[j]; j++);
        if(j==6) return 1;
        rotate(&d2, dir[i]);
    }
    return 0;
}

int main()
{
    dice d[100];
    int i, j, n;
    scanf("%d", &n);
    for(i=0; i<n; i++) {
        for(j=0; j<6; j++) {
            scanf("%d", &d[i].face[j]);
        }
        for(j=0; j<i; j++) {
            if(isSame(d[i], d[j])) {
                puts("No");
                return 0;
            }
        }
    }
    puts("Yes");
    return 0;
}
