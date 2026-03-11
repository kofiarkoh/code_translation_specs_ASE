#include <stdio.h>
#include <stdlib.h>
 
typedef struct{
    int num;
    char mark;
    int no;
}Card;

int partition(Card *A,int p, int r)
{
    int i, j;
    Card x, tmp;
    x = A[r];
    i = p - 1;
    for(j = p; j < r; j++)
    {
        if(A[j].num <= x.num)
        {
            i = i + 1;
            tmp = A[i];
            A[i] = A[j];
            A[j] = tmp;
        }
    }
    tmp = A[i + 1];
    A[i + 1] = A[r];
    A[r] = tmp;
    return i + 1;
}

void quickSort(Card *A,int p, int r)
{
    int q;
    if(p < r)
    {
        q = partition(A, p, r);
        quickSort(A, p, q - 1);
        quickSort(A, q + 1, r);
    }
}
 
int main()
{
    int n, i;
    scanf("%d",&n);
    Card *c = malloc(sizeof(Card)*n);
    for(i = 0; i < n; i++)
    {
        scanf(" %c%d",&c[i].mark,&c[i].num);
        c[i].no = i;
    }
    quickSort(c,0,n-1);
    for(i = 1; i < n; i++)
    {
        if((c[i-1].num == c[i].num) && (c[i-1].no > c[i].no))   break;
    }
    if(i == n) printf("Stable\n");
    else printf("Not stable\n");
    
    for(i = 0; i < n; i++)
        printf("%c %d\n",c[i].mark,c[i].num);
    return 0;
}
