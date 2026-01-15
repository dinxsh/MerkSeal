import { BatchRegistered } from "../generated/MerkleBatchRegistry/MerkleBatchRegistry"
import { Batch, BatchRegisteredEvent } from "../generated/schema"

export function handleBatchRegistered(event: BatchRegistered): void {
    // Create Batch entity
    let batch = new Batch(event.params.batchId.toString())
    batch.root = event.params.root
    batch.owner = event.params.owner
    batch.metaURI = event.params.metaURI
    batch.timestamp = event.block.timestamp
    batch.blockNumber = event.block.number
    batch.transactionHash = event.transaction.hash
    batch.save()

    // Create BatchRegisteredEvent entity
    let eventEntity = new BatchRegisteredEvent(
        event.transaction.hash.toHex() + "-" + event.logIndex.toString()
    )
    eventEntity.batchId = event.params.batchId
    eventEntity.root = event.params.root
    eventEntity.owner = event.params.owner
    eventEntity.metaURI = event.params.metaURI
    eventEntity.timestamp = event.block.timestamp
    eventEntity.blockNumber = event.block.number
    eventEntity.transactionHash = event.transaction.hash
    eventEntity.save()
}
