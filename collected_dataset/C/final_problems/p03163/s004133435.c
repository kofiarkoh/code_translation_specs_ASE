#include <stdio.h>

static struct {
        int weight;
        int value;
} items[100 + 1];

static long maxvalsum[100 + 1][10000 + 1];

static long knapsack (int const i, int const cap)
{
        long maxval, t = 0;
        if (i <= 0 || cap <= 0)
          return 0;
        if (maxvalsum[i][cap] >= 0)
                return maxvalsum[i][cap];
        
        maxval = knapsack(i - 1, cap);
        if (cap >= items[i].weight)
                t = knapsack(i - 1, cap - items[i].weight) + items[i].value;
        if (t > maxval)
                maxval = t;
        
        return maxvalsum[i][cap] = maxval;
}

int main (void)
{
        int item_count, knapsack_capacity, i, j;
        
        scanf("%d%d", &item_count, &knapsack_capacity);
        for (i = 1; i <= item_count; ++i)
                for (j = 1; j <= knapsack_capacity; ++j)
                        maxvalsum[i][j] = -1;
        for (i = 1; i <= item_count; ++i)
                scanf("%d%d", &items[i].weight, &items[i].value);
        
        printf("%ld\n", knapsack(item_count, knapsack_capacity));
        return 0;
}