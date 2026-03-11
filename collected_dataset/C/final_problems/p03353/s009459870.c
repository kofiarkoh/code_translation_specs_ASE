#include <stdio.h>
#include <stdlib.h>
#include <string.h>
char s[5005], ss[21001][21001], sn[3];
int k, fv[200], nr;
int partition (int low, int high)
{
    char pivot[5005];    // pivot
    strcpy(pivot, ss[high]);
    char aux[5005];
    int j;
    int i = (low - 1);  // Index of smaller element

    for (j = low; j <= high- 1; j++)
    {
        // If current element is smaller than or
        // equal to pivot
        if (strcmp(ss[j], pivot) <=0)
        {
            i++;    // increment index of smaller element
            strcpy(aux, ss[i]);
            strcpy(ss[i], ss[j]);
            strcpy(ss[j], aux);
        }
    }
    strcpy(aux, ss[i + 1]);
            strcpy(ss[i + 1], ss[high]);
            strcpy(ss[high], aux);
    return (i + 1);
}
void quickSort(int low, int high)
{
    if (low < high)
    {
        /* pi is partitioning index, arr[p] is now
           at right place */
        int pi = partition(low, high);

        // Separately sort elements before
        // partition and after partition
        quickSort(low, pi - 1);
        quickSort(pi + 1, high);
    }
}
void unificare()
{
    int i, j;
    for(i = 0; i < nr - 1; i ++)
        if(strcmp(ss[i], ss[i + 1]) == 0)
        {
        for(j = i + 1; j < nr - 1; j ++)
            strcpy(ss[j], ss[j + 1]);
        nr --;
        i --;
        }
}
int main()
{
    scanf("%[^\n]%*c", s);
    scanf("%d", &k);
    int i, j;
    for(i = 0; i < strlen(s); i ++)
        if(!fv[(int)s[i]])
        {
            sn[0] = s[i];
            strcpy(ss[nr ++], sn);
            fv[(int)s[i]] ++;
        }
   for(i = 0; i < strlen(s) - 1; i ++)
        for(j = i + 1; (j < strlen(s)) && ((j - i + 1) <= k); j ++)
            if((j - i + 1) <= k)
                strncpy(ss[nr ++], s + i, j - i + 1);
    quickSort(0, nr - 1);
    unificare();
    printf("%s", ss[k - 1]);
    /*for(i = 0; i < nr; i ++)
        printf("%s\n", ss[i]);*/
   return 0;
}
