import axios from "axios";

const API = "http://localhost:3000";

export interface Block {
  index: number;
  timestamp: string;
  data: string;
  previous_hash: string;
  hash: string;
}

// Fetch all blocks
export const getBlocks = async (): Promise<
  Block[]
> => {
  const response = await axios.get(
    `${API}/blocks`
  );

  return response.data;
};

// Add a new block
export const addBlock = async (
  data: string
) => {
  const response = await axios.post(
    `${API}/add`,
    {
      data,
    }
  );

  return response.data;
};

// Validate blockchain
export const validateBlockchain =
  async (): Promise<boolean> => {
    const response = await axios.get(
      `${API}/validate`
    );

    return response.data.valid;
  };